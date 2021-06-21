use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

pub type OpCodeType = u8;

#[derive(Serialize_repr, Deserialize_repr,PartialEq,Debug)]
#[repr(u8)]
pub enum Op {
    Dispatch = 0,
    Heartbeat = 1,
    Identify = 2,
    PUpd = 3,
    VSUpd = 4,
    Resume = 6,
    Reconnect = 7,
    ReqGuildMembers = 8,
    InvSession = 9,
    Hello = 10,
    HbACK = 11,
    Unknown = 255,
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

    pub fn to_code(&self) -> OpCodeType {
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

pub trait IsData {}

#[derive(Serialize, Deserialize,Debug)]
pub struct PartialPayload {
    pub op: Op,
}

#[derive(Serialize, Deserialize)]
pub struct Payload<T: IsData> {
    pub op: Op,
    pub d: T,
    pub s: Option<usize>,
    pub t: Option<String>,
}

impl<T: IsData> Payload<T> {
    pub fn new(d: T, s: Option<usize>, t: Option<String>, op: Op) -> Payload<T> {
        Payload { d, s, t, op }
    }
}

impl<T: Serialize + IsData> From<Payload<T>> for String {
    fn from(p: Payload<T>) -> Self {
        serde_json::to_string(&p).unwrap()
    }
}

pub mod data {
    use super::IsData;
    use serde::{Deserialize, Serialize};
    use serde_json::{Map, Value};

    #[derive(Serialize, Deserialize)]
    pub struct IdentifyProperties {
        #[serde(alias = "$os")]
        pub os: String,
        #[serde(alias = "$browser")]
        pub browser: String,
        #[serde(alias = "$device")]
        pub device: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Identify {
        pub token: String,
        pub properties: IdentifyProperties,
        pub intents: usize,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Resume {
        pub token: String,
        pub session_id: String,
        pub seq: usize,
    }

    pub type Heartbeat = Option<usize>;

    #[derive(Serialize, Deserialize)]
    pub struct UpdVState {
        pub guild_id: String,
        pub channel_id: Option<String>,
        pub self_mute: bool,
        pub self_deaf: bool,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Hello {
        pub heartbeat_interval: usize,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Ready {
        pub session_id: String,
    }

    pub type InvalidSession = bool;

    pub type Dispatch = Map<String, Value>;

    pub type HbACK = Option<bool>;

    impl IsData for Identify {}
    impl IsData for Resume {}
    impl IsData for UpdVState {}
    impl IsData for Heartbeat {}
    impl IsData for Hello {}
    impl IsData for Ready {}
    impl IsData for InvalidSession {}
    impl IsData for Dispatch {}
    impl IsData for HbACK {}
}
