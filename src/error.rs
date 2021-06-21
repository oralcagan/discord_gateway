use tokio::sync::broadcast;

#[derive(Debug)]
pub enum Error {
    HTTPError(reqwest::Error),
    EmptyField,
    WebSocketError(tokio_tungstenite::tungstenite::Error),
    DeserializeError(serde_json::Error),
    ChannelClosed,
    InvalidMessage
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::HTTPError(e)
    }
}

impl From<tokio_tungstenite::tungstenite::Error> for Error {
    fn from(e: tokio_tungstenite::tungstenite::Error) -> Self {
        match e {
            tokio_tungstenite::tungstenite::Error::ConnectionClosed => {
                panic!("This must be handled seperately")
            }
            tokio_tungstenite::tungstenite::Error::AlreadyClosed => {
                panic!("This must be handled seperately")
            }
            _ => {}
        };
        Error::WebSocketError(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::DeserializeError(e)
    }
}

impl<T> From<broadcast::error::SendError<T>> for Error {
    fn from(_: broadcast::error::SendError<T>) -> Self {
        Error::ChannelClosed
    }
}

impl From<broadcast::error::RecvError> for Error {
    fn from(_: broadcast::error::RecvError) -> Self {
        Error::ChannelClosed
    }
}
