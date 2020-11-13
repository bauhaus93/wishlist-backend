use std;
use thiserror::Error;

use crate::model::ErrorMessage;
use crate::persistence::Error as PersistenceError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("service/{source}")]
    Persistence {
        #[from]
        source: PersistenceError,
    },
}

impl Into<ErrorMessage> for Error {
    fn into(self) -> ErrorMessage {
        match self {
            Error::Persistence { source, .. } => source.into(),
        }
    }
}
