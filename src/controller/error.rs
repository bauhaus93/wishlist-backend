use std;
use thiserror::Error;
use warp;

use crate::model::ErrorMessage;
use crate::service::Error as ServiceError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("service/{source}")]
    Service {
        #[from]
        source: ServiceError,
    },
}

impl warp::reject::Reject for Error {}
