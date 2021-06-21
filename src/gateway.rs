use crate::{
    error::Error,
    events::{
        data::{HbACK, Heartbeat, Hello, Identify, IdentifyProperties},
        IsData, Op, PartialPayload, Payload,
    },
};
use futures::stream::{SplitSink, SplitStream};
use futures_util::{SinkExt, StreamExt};
use serde::{de::DeserializeOwned, Serialize};
use std::{collections::HashMap, convert::TryFrom, sync::Arc, time::Duration};
use tokio::{
    net::TcpStream,
    select, spawn,
    sync::{broadcast, Notify},
    task::JoinHandle,
    time::interval,
};
use tokio_tungstenite::{
    tungstenite::{
        protocol::{frame::coding::CloseCode, CloseFrame},
        Message,
    },
    MaybeTlsStream, WebSocketStream,
};

const API_VERSION: &str = "9";
const ENCODING: &str = "json";
const API: &str = "https://discord.com/api/gateway";
const BROADCAST_CAPACITY: usize = 50;
const GATEWAY_INTENTS: usize = 512;

async fn get_gateway_url(v: &str, enc: &str) -> Result<String, Error> {
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

fn close_msg(code: CloseCode) -> Message {
    Message::Close(Some(CloseFrame {
        code,
        reason: std::borrow::Cow::Borrowed(""),
    }))
}

pub trait GatewayEventHandler {}

pub struct Client<T: GatewayEventHandler> {
    gateway_url: String,
    session_id: Option<String>,
    seq_num: Option<usize>,
    token: String,
    event_handler: T,
    socket_handler: SocketHandler,
}

impl<T: GatewayEventHandler> Client<T> {
    pub async fn try_new(event_handler: T, token: String) -> Result<Self, Error> {
        Ok(Client {
            gateway_url: get_gateway_url(API_VERSION, ENCODING).await?,
            session_id: None,
            seq_num: None,
            token,
            event_handler,
            socket_handler: SocketHandler::new(),
        })
    }

    pub async fn run(&mut self) -> Result<(), Error> {
        self.connection_seq().await?;
        Ok(())
    }

    async fn connection_seq(&mut self) -> Result<SocketIO, Error> {
        let (w_h, r_h) = self.connect_to_ws().await?;
        let mut io = SocketIO::new(&self.socket_handler);
        let tasks = SocketTasks {
            reader: r_h,
            writer: w_h,
            writer_ch: self.socket_handler.get_writer(),
        };

        let hello = Payload::<Hello>::try_from(&io.r.read().await?.msg)?;
        let hb_not = Arc::new(Notify::new());
        let hearbeat_handle = make_hearbeat(
            hello.d.heartbeat_interval,
            SocketIO::new(&self.socket_handler),
            hb_not.clone(),
        );

        let identify = Payload::<Identify>::new(
            Identify {
                intents: GATEWAY_INTENTS,
                token: self.token.clone(),
                properties: IdentifyProperties {
                    browser: "".to_string(),
                    device: "".to_string(),
                    os: "".to_string(),
                },
            },
            None,
            None,
            Op::Identify,
        );
        io.w.write(identify)?;
        let msg = wait_for_payload(Op::Dispatch, &mut io).await?;
        println!("{:?}",msg);

        Err(Error::ChannelClosed)
    }

    async fn connect_to_ws(&self) -> Result<(JoinHandle<()>, JoinHandle<()>), Error> {
        let (stream, _) = tokio_tungstenite::connect_async(self.gateway_url.clone()).await?;
        let (writer, reader) = stream.split();

        let writer_handle = self.socket_handler.run_writer(writer).await;
        let reader_handle = self.socket_handler.run_reader(reader).await;

        Ok((writer_handle, reader_handle))
    }
}

type WsStream = WebSocketStream<MaybeTlsStream<TcpStream>>;

type SocketMsg = Message;

impl<T: IsData + DeserializeOwned> TryFrom<&SocketMsg> for Payload<T> {
    type Error = Error;

    fn try_from(value: &SocketMsg) -> Result<Self, Self::Error> {
        if let Message::Text(msg) = value {
            return Ok(serde_json::from_str::<Payload<T>>(&msg)?);
        }
        Err(Error::InvalidMessage)
    }
}

impl<T: IsData + Serialize> TryFrom<&Payload<T>> for SocketMsg {
    type Error = Error;

    fn try_from(p: &Payload<T>) -> Result<Self, Self::Error> {
        Ok(Message::Text(serde_json::to_string(p)?))
    }
}

impl TryFrom<&SocketMsg> for PartialPayload {
    type Error = Error;

    fn try_from(value: &SocketMsg) -> Result<Self, Self::Error> {
        if let Message::Text(msg) = value {
            return Ok(serde_json::from_str::<PartialPayload>(msg)?);
        }
        Err(Error::InvalidMessage)
    }
}

struct SocketHandler {
    broadcaster: broadcast::Sender<SocketMsg>,
    writer: broadcast::Sender<SocketMsg>,
}

impl SocketHandler {
    fn subscribe(&self) -> broadcast::Receiver<SocketMsg> {
        self.broadcaster.subscribe()
    }

    fn get_writer(&self) -> broadcast::Sender<SocketMsg> {
        self.writer.clone()
    }

    fn get_broadcaster(&self) -> broadcast::Sender<SocketMsg> {
        self.broadcaster.clone()
    }

    fn listen_writer(&self) -> broadcast::Receiver<SocketMsg> {
        self.writer.subscribe()
    }

    fn new() -> Self {
        let (broadcaster, _) = broadcast::channel::<SocketMsg>(BROADCAST_CAPACITY);
        let (writer, _) = broadcast::channel::<SocketMsg>(BROADCAST_CAPACITY);

        Self {
            broadcaster,
            writer,
        }
    }

    async fn run_reader(&self, mut reader: SplitStream<WsStream>) -> JoinHandle<()> {
        let broadcaster = self.get_broadcaster();
        spawn(async move {
            while let Some(res) = reader.next().await {
                match res {
                    Ok(msg) => {
                        broadcaster.send(Message::Text(msg.to_string()));
                    }
                    Err(err) => match err {
                        tokio_tungstenite::tungstenite::Error::Capacity(_) => (),
                        _ => panic!("Fatal: {}", err),
                    },
                }
            }
        })
    }

    async fn run_writer(&self, mut writer: SplitSink<WsStream, Message>) -> JoinHandle<()> {
        let mut listener = self.listen_writer();
        spawn(async move {
            loop {
                let msg = listener.recv().await.expect("Fatal");
                writer.send(msg).await.expect("Fatal");
            }
        })
    }
}

struct MsgWriter {
    writer: broadcast::Sender<SocketMsg>,
}

impl MsgWriter {
    fn new(writer: broadcast::Sender<SocketMsg>) -> Self {
        Self { writer }
    }

    fn write<T: IsData + Serialize>(&self, p: Payload<T>) -> Result<(), Error> {
        self.writer.send(Message::Text(String::from(p)))?;
        Ok(())
    }
}

#[derive(Debug)]
struct IncomingMsg {
    pp: PartialPayload,
    msg: Message,
}

struct MsgReader {
    reader: broadcast::Receiver<SocketMsg>,
}

impl MsgReader {
    fn new(reader: broadcast::Receiver<SocketMsg>) -> Self {
        Self { reader }
    }

    async fn read(&mut self) -> Result<IncomingMsg, Error> {
        let msg = self.reader.recv().await?;
        let pp = PartialPayload::try_from(&msg)?;
        Ok(IncomingMsg { msg, pp })
    }
}

struct SocketIO {
    w: MsgWriter,
    r: MsgReader,
}

impl SocketIO {
    fn new(sh: &SocketHandler) -> Self {
        let w = MsgWriter {
            writer: sh.get_writer(),
        };
        let r = MsgReader {
            reader: sh.subscribe(),
        };
        SocketIO { w, r }
    }
}

struct SocketTasks {
    writer: JoinHandle<()>,
    reader: JoinHandle<()>,
    writer_ch: broadcast::Sender<SocketMsg>,
}

impl Drop for SocketTasks {
    fn drop(&mut self) {
        self.writer_ch.send(close_msg(CloseCode::Normal));
        self.reader.abort();
    }
}

fn make_hearbeat(intv: usize, mut io: SocketIO, notify: Arc<Notify>) -> JoinHandle<()> {
    spawn(async move {
        let mut interval = interval(Duration::from_millis(intv as u64));
        interval.tick().await;
        loop {
            io.w.write(Payload::<Heartbeat>::new(None, None, None, Op::Heartbeat))
                .expect("Fatal");
            let waiter = wait_for_payload(Op::Heartbeat, &mut io);
            let tick = interval.tick();
            loop {
                select! {
                    r = waiter => {
                        if r.is_err() {
                            notify.notify_one();
                            panic!("Fatal")
                        }
                        break
                    },
                    _ = tick => {
                        notify.notify_one();
                        panic!("Fatal")
                    }
                }
            }
        }
    })
}

async fn wait_for_payload(op: Op, io: &mut SocketIO) -> Result<IncomingMsg, Error> {
    loop {
        let p = io.r.read().await?;
        if op == p.pp.op {
            return Ok(p);
        }
    }
}
