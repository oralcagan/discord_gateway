use crate::error::Error;
use futures::{SinkExt, StreamExt};
use std::collections::HashMap;
use tokio::net::TcpStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};

const API_VERSION: &str = "9";
const ENCODING: &str = "json";
const API: &str = "https://discord.com/api";

async fn _get_gateway_url(v: &str, enc: &str) -> Result<String, Error> {
    let map = reqwest::get(API)
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    let mut url = match map.get("url") {
        Some(s) => s.clone(),
        None => return Err(crate::error::Error::EmptyField),
    };
    url.push_str("?v=");
    url.push_str(v);
    url.push_str("&encoding=");
    url.push_str(enc);
    Ok(url)
}

trait GatewayEventHandler {}
struct DefaultHandler {}
impl GatewayEventHandler for DefaultHandler {}

struct Client<T: GatewayEventHandler> {
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

        Err(crate::error::Error::EmptyField)
    }

    async fn connection_seq(&self) -> Result<WebSocketStream<MaybeTlsStream<TcpStream>>, Error> {
        let (mut connection, _) = tokio_tungstenite::connect_async(self.gateway_url).await?;

    }

    fn resume_seq(&self) {}
}