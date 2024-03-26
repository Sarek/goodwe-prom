#[derive(Debug)]
pub enum DecodeError {
  MessageTypeNotImplemented,
  InvalidHeader,
}

pub fn decode_response(data: &[u8]) -> Result<super::ResponseMessage, DecodeError> {
  let mut data = data.into_iter();

  if data.next() != Some(&0xaa) || data.next() != Some(&0x55) {
    return Err(DecodeError::InvalidHeader);
  }

  Err(DecodeError::MessageTypeNotImplemented)
}
