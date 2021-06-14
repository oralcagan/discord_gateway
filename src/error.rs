pub enum Error {
    HTTPError(reqwest::Error),
    EmptyField
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::HTTPError(e)
    }
}