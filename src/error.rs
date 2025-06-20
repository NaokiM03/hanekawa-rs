#[derive(Debug)]
pub enum Error {
    Rusqlite(rusqlite::Error),
    SerdeJson(serde_json::Error),
    Time(std::time::SystemTimeError),
}

pub type Result<T> = core::result::Result<T, Error>;

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Error::Rusqlite(err) => core::fmt::Display::fmt(err, f),
            Error::SerdeJson(err) => core::fmt::Display::fmt(err, f),
            Error::Time(err) => core::fmt::Display::fmt(err, f),
        }
    }
}

impl core::error::Error for Error {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            Error::Rusqlite(err) => Some(err),
            Error::SerdeJson(err) => Some(err),
            Error::Time(err) => Some(err),
        }
    }
}

impl core::convert::From<rusqlite::Error> for Error {
    fn from(source: rusqlite::Error) -> Self {
        Error::Rusqlite(source)
    }
}

impl core::convert::From<serde_json::Error> for Error {
    fn from(source: serde_json::Error) -> Self {
        Error::SerdeJson(source)
    }
}

impl core::convert::From<std::time::SystemTimeError> for Error {
    fn from(source: std::time::SystemTimeError) -> Self {
        Error::Time(source)
    }
}
