use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

type OpCodeType = u8;

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
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

pub trait IsData {}

#[derive(Serialize, Deserialize)]
struct Payload<T: IsData> {
    op: Op,
    d: Option<T>,
    s: Option<usize>,
    t: Option<String>,
}

pub mod data {
    use super::IsData;
    use serde::{Deserialize, Serialize};
    
    #[derive(Serialize, Deserialize)]
    struct IdentifyProperties {
        #[serde(alias = "$os")]
        os: String,
        #[serde(alias = "$browser")]
        browser: String,
        #[serde(alias = "$device")]
        device: String,
    }

    #[derive(Serialize, Deserialize)]
    struct Identify {
        token: String,
        properties: IdentifyProperties,
        intents: u32,
    }

    #[derive(Serialize, Deserialize)]
    struct Resume {
        token: String,
        session_id: String,
        seq: usize,
    }

    type Heartbeat = usize;

    #[derive(Serialize, Deserialize)]
    struct UpdVState {
        guild_id: String,
        channel_id: Option<String>,
        self_mute: bool,
        self_deaf: bool,
    }

    #[derive(Serialize, Deserialize)]
    struct Hello {
        heartbeat_interval: usize,
    }

    #[derive(Serialize, Deserialize)]
    struct Ready {
        session_id: String,
    }

    type InvalidSession = bool;

    impl IsData for Identify {}
    impl IsData for Resume {}
    impl IsData for UpdVState {}
    impl IsData for Heartbeat {}
    impl IsData for Hello {}
    impl IsData for Ready {}
    impl IsData for InvalidSession {}
}
