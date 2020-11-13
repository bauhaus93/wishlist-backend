use mongodb::sync::Client;

use super::get_mongo_client;
use crate::model::Wishlist;
use crate::persistence::{Error, Result, WishlistDao};

pub struct MongoWishlistDao {
    client: Client,
}

impl MongoWishlistDao {
    pub fn new() -> Result<MongoWishlistDao> {
        Ok(MongoWishlistDao {
            client: get_mongo_client()?,
        })
    }
}

impl WishlistDao for MongoWishlistDao {
    fn get_last_wishlist(&self) -> Result<Wishlist> {
        Err(Error::EmptyResult)
    }
}
