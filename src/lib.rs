extern crate mongodb;
extern crate warp;
#[macro_use]
extern crate log;
extern crate bson;
extern crate thiserror;

mod controller;
mod error;
mod model;
mod persistence;
mod reject;
mod routes;
mod service;

pub use self::error::{Error, Result};
pub use self::routes::create_routes;
