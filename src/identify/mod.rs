use std::{fmt::Display, net::UdpSocket, str::from_utf8, time::Duration};

const ID_QUERY: [u8; 9] = [0xaa, 0x55, 0xc0, 0x7f, 0x01, 0x02, 0x00, 0x02, 0x41];

pub struct IdResponse {
    pub serial_number: String,
    pub firmware: String,
}

pub enum RequestError {
    NetworkError(std::io::Error),
    NoResponse,
    InvalidResponse(String),
}

impl Display for RequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RequestError::NetworkError(ioe) => write!(f, "Network Error: {ioe}"),
            RequestError::NoResponse => write!(f, "No response received"),
            RequestError::InvalidResponse(reason) => {
                write!(f, "Response deemed invalid due to {reason}")
            }
        }
    }
}

fn map_network_error(e: std::io::Error) -> RequestError {
    RequestError::NetworkError(e)
}

fn decode_response(data: &[u8]) -> Result<IdResponse, RequestError> {
    let mut data = data.iter();

    if data.next() != Some(&0xaa) || data.next() != Some(&0x55) {
        return Err(RequestError::InvalidResponse("Header".to_string()));
    }

    if data.next() != Some(&0x7f) {
        return Err(RequestError::InvalidResponse("Source Address".to_string()));
    }

    if data.next() != Some(&0xc0) {
        return Err(RequestError::InvalidResponse("Target Address".to_string()));
    }

    if data.next() != Some(&0x01) {
        return Err(RequestError::InvalidResponse("Control Code".to_string()));
    }

    if data.next() != Some(&0x82) {
        return Err(RequestError::InvalidResponse("Function Code".to_string()));
    }

    // Contrary to other information, it appears that my GW20K-ET
    // inverter uses a different protocol/message content than expected.
    // The only identifiable information is the serial number and the
    // internal firmware version.
    // As this is the only inverter I have, I won't implement anything
    // else for now. If you have data from other models, you're very
    // welcome to provide it to me, I will extend the code as needed.

    let length = data.next();
    if length != Some(&76) || data.len() != (76 + 2) {
        return Err(RequestError::InvalidResponse("Length".to_string()));
    }

    let _ = data.advance_by(31);
    let mut serial_number = Vec::new();
    for elem in data.next_chunk::<16>().unwrap().as_slice() {
        serial_number.push(**elem);
    }
    let serial_number = from_utf8(serial_number.as_slice()).unwrap().to_owned();

    let _ = data.advance_by(17);
    let mut firmware = Vec::new();
    for elem in data.next_chunk::<10>().unwrap().as_slice() {
        firmware.push(**elem);
    }
    let firmware = from_utf8(firmware.as_slice()).unwrap().to_owned();

    Ok(IdResponse {
        serial_number,
        firmware,
    })
}

pub fn query_id(target: &str) -> Result<IdResponse, RequestError> {
    let sock = UdpSocket::bind("0.0.0.0:0").map_err(map_network_error)?;
    sock.connect(target).map_err(map_network_error)?;
    sock.send(&ID_QUERY).map_err(map_network_error)?;
    sock.set_read_timeout(Some(Duration::from_secs(3)))
        .map_err(map_network_error)?;

    let mut buf = [0; 128];
    match sock.recv(&mut buf) {
        Ok(size) => decode_response(&buf[0..size]),
        Err(_) => Err(RequestError::NoResponse),
    }
}
