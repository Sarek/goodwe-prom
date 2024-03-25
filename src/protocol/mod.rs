use std::net::IpAddr;

//
// For documentation of the protocol, see https://yamasun.com.tw/upload/F_20170313191367UrC8jo.PDF
//

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

      _ => panic!("Unknown response type: {:#06x}", response_type)
    }
  }
}
