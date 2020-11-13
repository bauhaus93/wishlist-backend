use std;
use thiserror::Error;

use crate::controller::Error as ControllerError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("controller: {source}")]
    Controller {
        #[from]
        source: ControllerError,
    },
}
