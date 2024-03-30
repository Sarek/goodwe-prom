use std::{error::Error, fmt::Display};

use crc16::{MODBUS,State};

pub const DEFAULT_ADDR: u8 = 0x7f;

pub enum Command {
  ReadMulti = 0x03,
  WriteSingle = 0x06,
  WriteMulti = 0x10,
}

#[derive(Debug)]
pub enum ModbusError {
  InvalidHeader,
  WrongChecksum,
  FailedCommand,
  PayloadLength,
}

impl Display for ModbusError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ModbusError::InvalidHeader => write!(f, "Invalid Header"),
            ModbusError::WrongChecksum => write!(f, "CRC-16 checksum wrong, data was corrupted"),
            ModbusError::FailedCommand => write!(f, "Command failed"),
            ModbusError::PayloadLength => write!(f, "Indicated payload length does not match actual length"),
        }
    }
}

impl Error for ModbusError {}

fn crc(data: &[u8]) -> Vec<u8> {
  let checksum = State::<MODBUS>::calculate(data);

  let mut retval: Vec<u8> = Vec::new();
  retval.push((checksum & 0xff) as u8);
  retval.push(((checksum >> 8) & 0xff) as u8);
  retval
}

pub fn create_command(cmd: Command, addr: u8, reg: u16, param: u16) -> Vec<u8> {
  let mut data: Vec<u8> = Vec::new();

  data.push(cmd as u8);
  data.push(addr);
  data.push(((reg >> 8) & 0xff) as u8);
  data.push((reg & 0xff) as u8);
  data.push(((param >> 8) & 0xff) as u8);
  data.push((param & 0xff) as u8);

  data.append(&mut crc(&data));
  data
}

pub fn get_payload(data: &Vec<u8>) -> Result<Vec<u8>, ModbusError> {
  // We do not get real Modbus packets back, but they look like AA55 protocol packets
  // Let's validate them anyway :-)

  // CRC
  if State::<MODBUS>::calculate(data) != 0 {
    return Err(ModbusError::WrongChecksum);
  }

  let mut data = data.iter();
  
  // We expect the AA55 header
  if (data.next() != Some(&0xaa)) || (data.next() != Some(&0x55)) {
    return Err(ModbusError::InvalidHeader);
  };

  let _ = data.next(); // we don't care about the communication address

  // Modbus Command. If the highest bit is set to 1, the command failed
  let cmd = data.next();
  if !cmd.is_some() || ((cmd.unwrap() & 0x80) > 0) {
    return Err(ModbusError::FailedCommand);
  }

  // Does the actual remaining length match the advertised payload length
  let payload_length = (data.len() - 3) as u8; // length byte + payload + CRC
  if data.next() != Some(&(payload_length - 3)) {
    return Err(ModbusError::PayloadLength);
  }

  // TODO: This is most likely not optimal, and certainly not very nice.
  let mut retval: Vec<u8> = Vec::new();
  while payload_length > 0 {
      retval.push(data.next().unwrap().clone());
  }
  Ok(retval)
}
