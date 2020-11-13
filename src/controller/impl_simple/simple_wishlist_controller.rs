use warp;

use crate::controller::{Error, Result, WishlistController};
use crate::model::Wishlist;
use crate::service::{SimpleWishlistService, WishlistService};

pub struct SimpleWishlistController {
    wishlist_service: Box<dyn WishlistService>,
}

impl SimpleWishlistController {
    pub fn new() -> Result<Self> {
        Ok(Self {
            wishlist_service: Box::new(SimpleWishlistService::new()?),
        })
    }
}

impl WishlistController for SimpleWishlistController {
    fn get_last_wishlist(&self) -> Result<Wishlist> {
        self.wishlist_service
            .get_last_wishlist()
            .map_err(Error::from)
    }
}
