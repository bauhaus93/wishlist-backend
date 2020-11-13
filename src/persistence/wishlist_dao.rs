use super::Result;
use crate::model::Wishlist;

pub trait WishlistDao: Send + Sync {
    fn get_last_wishlist(&self) -> Result<Wishlist>;
}
