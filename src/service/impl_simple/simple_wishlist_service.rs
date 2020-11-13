use crate::model::Wishlist;
use crate::persistence::{MongoWishlistDao, WishlistDao};
use crate::service::{Error, Result, WishlistService};

pub struct SimpleWishlistService {
    wishlist_dao: Box<dyn WishlistDao>,
}

impl SimpleWishlistService {
    pub fn new() -> Result<SimpleWishlistService> {
        Ok(Self {
            wishlist_dao: Box::new(MongoWishlistDao::new()?),
        })
    }
}

impl WishlistService for SimpleWishlistService {
    fn get_last_wishlist(&self) -> Result<Wishlist> {
        self.wishlist_dao.get_last_wishlist().map_err(Error::from)
    }
}
