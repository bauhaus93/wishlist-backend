use std;
use std::convert::Infallible;
use std::sync::Arc;
use warp;
use warp::http::StatusCode;
use warp::Filter;

use super::Error as ApplicationError;
use super::Result;
use crate::controller::{SimpleWishlistController, WishlistController};
use crate::model::ErrorMessage;

macro_rules! reply_future {
    ($controller:ident, $method:ident) => {{
        |controller: Arc<dyn $controller>| async move {
            match controller.$method() {
                Ok(output) => Ok(warp::reply::json(&output)),
                Err(e) => Err(warp::reject::custom(e)),
            }
        }
    }};
}

pub async fn create_routes() -> Result<impl warp::Filter<Extract = impl warp::Reply> + Clone> {
    let wishlist_controller: Arc<dyn WishlistController> =
        Arc::new(SimpleWishlistController::new()?);

    let wishlist_controller_filter = warp::any().map(move || wishlist_controller.clone());
    let log_filter = warp::log("api");

    let route_get_last_wishlist = warp::get()
        .and(warp::path("api"))
        .and(warp::path("wishlist"))
        .and(warp::path::end())
        .and(wishlist_controller_filter.clone())
        .and_then(reply_future!(WishlistController, get_last_wishlist));

    let routes = route_get_last_wishlist
        .with(log_filter)
        .recover(handel_rejection);

    Ok(routes)
}

async fn handel_rejection(
    rej: warp::Rejection,
) -> std::result::Result<impl warp::Reply, Infallible> {
    let code;
    let message;
    if rej.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = "Page not found".to_string();
    } else if let Some(err) = rej.find::<ApplicationError>() {
        match err {
            ApplicationError::Controller { .. } => {
                code = StatusCode::INTERNAL_SERVER_ERROR;
                message = "Internal error".to_string();
                error!("{}", err);
            }
        }
    } else if let Some(err) = rej.find::<warp::filters::body::BodyDeserializeError>() {
        info!("{}", err);
        code = StatusCode::BAD_REQUEST;
        message = "Bad request: Could not deserialize JSON".to_string();
    } else {
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "Unhandeled internal error".to_string();
    }
    let json = warp::reply::json(&ErrorMessage {
        code: code.as_u16(),
        message: message.into(),
    });
    Ok(warp::reply::with_status(json, code))
}
