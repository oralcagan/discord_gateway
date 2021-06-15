use crate::error::Error;
use std::collections::HashMap;
use futures_util::{StreamExt,SinkExt};

const API_VERSION: &str = "9";
const ENCODING: &str = "json";
const API: &str = "https://discord.com/api/gateway";

async fn _get_gateway_url(v: &str, enc: &str) -> Result<String, Error> {
    let map = reqwest::get(API)
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    let mut url = match map.get("url") {
        Some(s) => s.clone(),
        None => return Err(Error::EmptyField),
    };
    url.push_str("/?v=");
    url.push_str(v);
    url.push_str("&encoding=");
    url.push_str(enc);
    Ok(url)
}

pub trait GatewayEventHandler {}

pub struct Client<T: GatewayEventHandler> {
    gateway_url: String,
    session_id: String,
    seq_num: Option<usize>,
    heartbeat_int: usize,
    hb_acked: bool,
    token: String,
    handler: T,
}

impl<T: GatewayEventHandler> Client<T> {
    pub fn new(handler: T) -> Self {
        Client {
            gateway_url: String::from(""),
            session_id: String::from(""),
            seq_num: None,
            heartbeat_int: 0,
            hb_acked: true,
            token: String::from(""),
            handler,
        }
    }

    pub async fn run(handler: T, token: String) -> Result<(), Error> {
        let url = _get_gateway_url(API_VERSION, ENCODING).await?;
        let mut client = Client::new(handler);
        client.gateway_url = url;
        client.token = token;

        let (mut con,_) = tokio_tungstenite::connect_async(client.gateway_url.clone()).await?;


        Err(crate::error::Error::EmptyField)
    }

    async fn connection_seq(&mut self) -> Result<(), Error> {
        Err(Error::EmptyField)
    }

    fn resume_seq(&self) {}
}
