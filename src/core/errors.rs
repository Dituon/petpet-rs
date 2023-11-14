#[derive(Debug)]
pub enum Error<'a> {
    RequestError(reqwest::Error),
    ImageDecodeError(&'a str),
    FileError(&'a str),
    TemplateError(&'a str),
    AvatarLoadError(&'a str),
    EvalPosError(meval::Error),
    MissingDataError(&'a str),
    SyncPoisonError(std::sync::PoisonError<&'a str>)
}

impl<'a> std::fmt::Display for Error<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::RequestError(err) => write!(f, "Request error: {}", err),
            Error::ImageDecodeError(msg) => write!(f, "Decode error: {}", msg),
            Error::FileError(msg) => write!(f, "File error: {}", msg),
            Error::TemplateError(msg) => write!(f, "Template error: {}", msg),
            Error::AvatarLoadError(msg) => write!(f, "Avatar load error: {}", msg),
            Error::EvalPosError(err) => write!(f, "Eval pos error: {}", err),
            Error::MissingDataError(msg) => write!(f, "Missing data error: {}", msg),
            Error::SyncPoisonError(err) => {write!(f, "Sync poison error: {}", err)}
        }
    }
}

impl<'a> std::error::Error for Error<'a> {}

impl From<reqwest::Error> for Error<'_> {
    fn from(error: reqwest::Error) -> Self {
        Error::RequestError(error)
    }
}

impl From<meval::Error> for Error<'_> {
    fn from(error: meval::Error) -> Self {
        Error::EvalPosError(error)
    }
}

impl<T> From<std::sync::PoisonError<T>> for Error<'_> {
    fn from(_error: std::sync::PoisonError<T>) -> Self {
        Error::SyncPoisonError(std::sync::PoisonError::new(""))
    }
}