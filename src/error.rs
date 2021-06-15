#[derive(Debug)]
pub enum Error {
    HTTPError(reqwest::Error),
    EmptyField,
    WebSocketError(tokio_tungstenite::tungstenite::Error)
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::HTTPError(e)
    }
}

impl From<tokio_tungstenite::tungstenite::Error> for Error {
    fn from(e: tokio_tungstenite::tungstenite::Error) -> Self {
        match e {
            tokio_tungstenite::tungstenite::Error::ConnectionClosed => panic!("This must be handled seperately"),
            tokio_tungstenite::tungstenite::Error::AlreadyClosed => panic!("This must be handled seperately"),
            _ => {}
        };
        Error::WebSocketError(e)
    }
}