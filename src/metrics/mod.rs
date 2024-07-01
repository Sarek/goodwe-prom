use std::{
    fmt::Display,
    net::{Ipv4Addr, UdpSocket},
    time::Duration,
};

use self::modbus::ModbusError;

mod definitions;
mod modbus;

pub mod et;

pub enum MetricsError {
    #[allow(dead_code)]
    MetricReadError(definitions::MetricReadError),
    ModbusError(ModbusError),
    NetworkError(std::io::Error),
}

impl Display for MetricsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            MetricsError::MetricReadError(e) => write!(f, "Metric Read Error: {}", e),
            MetricsError::NetworkError(e) => write!(f, "Network Error: {}", e),
            MetricsError::ModbusError(e) => write!(f, "Modbus Error: {}", e),
        }
    }
}

pub type MetricSet = definitions::MetricSet;

pub fn get_metrics(target: &str, ms: &mut MetricSet) -> Result<(), MetricsError> {
    let mut target = target.to_owned();
    target.push_str(":8899");
    let cmd = ms.get_modbus_command(modbus::DEFAULT_ADDR);

    let sock = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, 0)).map_err(map_network_error)?;
    sock.connect(target).map_err(map_network_error)?;
    sock.send(&cmd).map_err(map_network_error)?;
    sock.set_read_timeout(Some(Duration::from_secs(3)))
        .map_err(map_network_error)?;

    let mut buf = [0; 1024];
    let _ = match sock.recv(&mut buf) {
        Ok(size) => match modbus::get_payload(&buf[0..size].to_vec()) {
            Ok(data) => ms.read_data(&data),
            Err(e) => return Err(map_modbus_error(e)),
        },
        Err(e) => return Err(map_network_error(e)),
    };

    Ok(())
}

fn map_network_error(e: std::io::Error) -> MetricsError {
    MetricsError::NetworkError(e)
}

fn map_modbus_error(e: ModbusError) -> MetricsError {
    MetricsError::ModbusError(e)
}
