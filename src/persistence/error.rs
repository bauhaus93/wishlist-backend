use std;
use thiserror::Error;

use bson::document::ValueAccessError;
use bson::oid::Error as BsonError;
use mongodb::error::Error as MongoError;

use crate::model::ErrorMessage;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("MongoDB: {source}")]
    MongoDB {
        #[from]
        source: MongoError,
    },
    #[error("BSON: {source}")]
    Bson {
        #[from]
        source: BsonError,
    },
    #[error("ValueAccess: {source}")]
    ValueAccess {
        #[from]
        source: ValueAccessError,
    },
    #[error("Received an empty result")]
    EmptyResult,
    #[error("Can't connect to database: Environment variable 'DATABASE_URL' not set!")]
    DatabaseURLNotSet,
}

impl Into<ErrorMessage> for &Error {
    fn into(self) -> ErrorMessage {
        let code;
        let message;
        match self {
            Error::EmptyResult => {
                code = 200;
                message = "Empty Result";
            }
            _ => {
                code = 500;
                message = "Internal Error";
            }
        }
        ErrorMessage {
            code: code,
            message: message.to_string(),
        }
    }
}
