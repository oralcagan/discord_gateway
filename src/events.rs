use crate::payload::PayloadData;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Ready {
    pub session_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub channel_id: String
}

impl PayloadData for Ready {}
impl PayloadData for Message {}