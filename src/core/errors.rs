#[derive(Debug)]
pub enum Error {
    RequestError(reqwest::Error),
    UrlParseError(url::ParseError),
    ImageDecodeError(String),
    ImageSynthesisError(String),
    ImageEncodeError(String),
    FileError(String),
    TemplateError(String),
    AvatarLoadError(String),
    EvalPosError(meval::Error),
    MissingDataError(String),
    SyncPoisonError(std::sync::PoisonError<String>),
    SerializationError(serde_json::Error),
    IOError(std::io::Error),
}

impl<'a> std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::RequestError(err) => write!(f, "Request error: {}", err),
            Error::UrlParseError(err) => write!(f, "URL Parse error: {}", err),
            Error::ImageDecodeError(msg) => write!(f, "Decode error: {}", msg),
            Error::ImageSynthesisError(msg) => write!(f, "Synthesis error: {}", msg),
            Error::ImageEncodeError(msg) => write!(f, "Encode error: {}", msg),
            Error::FileError(msg) => write!(f, "File error: {}", msg),
            Error::TemplateError(msg) => write!(f, "Template error: {}", msg),
            Error::AvatarLoadError(msg) => write!(f, "Avatar load error: {}", msg),
            Error::EvalPosError(err) => write!(f, "Eval pos error: {}", err),
            Error::MissingDataError(msg) => write!(f, "Missing data error: {}", msg),
            Error::SyncPoisonError(err) => write!(f, "Sync poison error: {}", err),
            Error::SerializationError(err) => write!(f, "Serialization error: {}", err),
            Error::IOError(err) => write!(f, "IO error: {}", err),
        }
    }
}

impl<'a> std::error::Error for Error {}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Error::RequestError(error)
    }
}

impl From<url::ParseError> for Error {
    fn from(error: url::ParseError) -> Self {
        Error::UrlParseError(error)
    }
}

impl From<meval::Error> for Error {
    fn from(error: meval::Error) -> Self {
        Error::EvalPosError(error)
    }
}

impl From<skia_safe::codec::Result> for Error {
    fn from(_: skia_safe::codec::Result) -> Self {
        Error::ImageDecodeError("".to_string())
    }
}

impl<T> From<std::sync::PoisonError<T>> for Error {
    fn from(_error: std::sync::PoisonError<T>) -> Self {
        Error::SyncPoisonError(std::sync::PoisonError::new("".to_string()))
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Error::SerializationError(error)
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::IOError(error)
    }
}