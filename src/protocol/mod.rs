use std::{net::UdpSocket, time::Duration};

mod decoder;

//
// For documentation of the protocol, see https://yamasun.com.tw/upload/F_20170313191367UrC8jo.PDF
//

#[derive(Debug)]
pub enum RequestMessage {
    // Control Code 0x00 - Register
    OfflineQuery,
    AllocateRegisterAddress,
    RemoveRegister,
    // Control Code 0x01 - Read
    QueryRunningInfo,
    QueryIdInfo,
    QuerySettingInfo,
    // Control Code 0x03 - Execute
    StartInverter,
    StopInverter,
    DisconnectGridAndReconnect,
    AdjustRealPower,
}

#[derive(Debug)]
pub enum ResponseMessage {
    // Control Code 0x00 - Register
    RegisterRequest,
    AddressConfirm,
    RemoveConfirm,
    // Control Code 0x01 - Read
    ResponseRunningInfo,
    ResponseIdInfo,
    ResponseSettingInfo,
    // Control Code 0x03 - Execute
    StartInverterResponse,
    StopInverterResponse,
    DisconnectGridAndReconnectResponse,
    AdjustRealPowerResponse,
}

impl RequestMessage {
    pub fn value(&self) -> &[u8] {
        match *self {
            // Control Code 0x00 - Register
            RequestMessage::OfflineQuery => &[0x00, 0x00],
            RequestMessage::AllocateRegisterAddress => &[0x00, 0x01],
            RequestMessage::RemoveRegister => &[0x00, 0x02],
            // Control Code 0x01 - Read
            RequestMessage::QueryRunningInfo => &[0x01, 0x01],
            RequestMessage::QueryIdInfo => &[0x01, 0x02],
            RequestMessage::QuerySettingInfo => &[0x01, 0x03],
            // Control Code 0x03 - Execute
            RequestMessage::StartInverter => &[0x03, 0x1b],
            RequestMessage::StopInverter => &[0x03, 0x1C],
            RequestMessage::DisconnectGridAndReconnect => &[0x03, 0x1D],
            RequestMessage::AdjustRealPower => &[0x03, 0x1E],
        }
    }
}

impl ResponseMessage {
    pub fn from(response_type: &u16) -> ResponseMessage {
        match response_type {
            0x0080 => ResponseMessage::RegisterRequest,
            0x0081 => ResponseMessage::AddressConfirm,
            0x0082 => ResponseMessage::RemoveConfirm,

            0x0181 => ResponseMessage::ResponseRunningInfo,
            0x0182 => ResponseMessage::ResponseIdInfo,
            0x0183 => ResponseMessage::ResponseSettingInfo,

            0x039b => ResponseMessage::StartInverterResponse,
            0x039c => ResponseMessage::StopInverterResponse,
            0x039d => ResponseMessage::DisconnectGridAndReconnectResponse,
            0x039e => ResponseMessage::AdjustRealPowerResponse,

            _ => panic!("Unknown response type: {:#06x}", response_type),
        }
    }
}

pub fn send_request(target: &str, msg: RequestMessage) -> std::io::Result<()> {
    let mut raw_message = vec![0xAA, 0x55, 0xc0, 0x7f]; // 2 bytes constant header, 1 byte source, 1 byte destination (also constant)
    raw_message.extend_from_slice(msg.value());
    raw_message.push(0x00); // data length
    let checksum: u16 = raw_message.iter().fold(0, |accu, x| accu + (*x as u16));
    raw_message.push((checksum >> 8) as u8);
    raw_message.push((checksum & 0x00ff) as u8);

    match send_await_response(target, raw_message) {
        Err(e) => {
            println!("Error when sending/receiving request: {}", e);
        }
        Ok(response) => {
            println!("Received response message: {:?}", response);
        }
    }
    Ok(())
}

fn send_await_response(target: &str, request: Vec<u8>) -> std::io::Result<ResponseMessage> {
    let sock = UdpSocket::bind("0.0.0.0:0")?;
    sock.connect(target)?;
    sock.send(&request)?;
    sock.set_read_timeout(Some(Duration::from_secs(3)))?;

    let mut buf = [0; 128];
    match sock.recv(&mut buf) {
        Ok(size) => match decoder::decode_response(&buf[0..size]) {
            Ok(msg) => return Ok(msg),
            Err(e) => {
                println!("Failed to decode response message: {:?}", e);
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Decoding error",
                ));
            }
        },
        Err(_) => {
            println!("No or faulty response received!");
            return Err(std::io::Error::new(
                std::io::ErrorKind::TimedOut,
                "Timeout waiting for a response",
            ));
        }
    }
}
