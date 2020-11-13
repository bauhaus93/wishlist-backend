use std;
use thiserror::Error;

use bson::document::ValueAccessError;
use bson::oid::Error as BsonError;
use mongodb::error::Error as MongoError;

use crate::model::ErrorMessage;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("mongodb: {source}")]
    MongoDB {
        #[from]
        source: MongoError,
    },
    #[error("bson: {source}")]
    Bson {
        #[from]
        source: BsonError,
    },
    #[error("valueaccess: {source}")]
    ValueAccess {
        #[from]
        source: ValueAccessError,
    },
    #[error("Got an empty result")]
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
                code = 204;
                message = "No Content";
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
