use crate::error::Error;
use std::collections::HashMap;

type OpCodeType = u8;

enum Op {
    Dispatch,
    Heartbeat,
    Identify,
    PUpd,
    VSUpd,
    Resume,
    Reconnect,
    ReqGuildMembers,
    InvSession,
    Hello,
    HbACK,
    Unknown,
}

impl Op {
    pub fn from_code(c: OpCodeType) -> Self {
        match c {
            0 => Self::Dispatch,
            1 => Self::Heartbeat,
            2 => Self::Identify,
            3 => Self::PUpd,
            4 => Self::VSUpd,
            6 => Self::Resume,
            7 => Self::Reconnect,
            8 => Self::ReqGuildMembers,
            9 => Self::InvSession,
            10 => Self::Hello,
            11 => Self::HbACK,
            _ => Self::Unknown,
        }
    }

    fn to_code(&self) -> OpCodeType {
        match self {
            Op::Dispatch => 0,
            Op::Heartbeat => 1,
            Op::Identify => 2,
            Op::PUpd => 3,
            Op::VSUpd => 4,
            Op::Resume => 6,
            Op::Reconnect => 7,
            Op::ReqGuildMembers => 8,
            Op::InvSession => 9,
            Op::Hello => 10,
            Op::HbACK => 11,
            Op::Unknown => panic!("Unknown opcode check"),
        }
    }
}

const API: &str = "https://discord.com/api";

async fn _get_gateway_url(v: &str, enc: &str) -> Result<String, Error> {
    let map = reqwest::get(API)
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    let mut url = match map.get("url") {
        Some(s) => s.clone(),
        None => return Err(Error::EmptyField),
    };
    url.push_str("?v=");
    url.push_str(v);
    url.push_str("&encoding=");
    url.push_str(enc);
    Ok(url)
}

trait GatewayEventHandler {

}