use lapin::Error as LapinError;
use thiserror::Error;
use tokio::sync::mpsc::error::SendError;

#[derive(Error, Debug)]
pub enum Error {
    #[error("rabbit error: {0}")]
    Rabbit(LapinError),
    #[error("queue error")]
    Mpsc,
}

impl From<LapinError> for Error {
    fn from(err: LapinError) -> Self {
        Self::Rabbit(err)
    }
}

impl<T> From<SendError<T>> for Error {
    fn from(_err: SendError<T>) -> Self {
        Self::Mpsc
    }
}
