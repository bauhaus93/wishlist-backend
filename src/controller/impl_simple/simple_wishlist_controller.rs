use lru_time_cache::LruCache;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use crate::controller::{Error, Result, WishlistController};
use crate::model::Wishlist;
use crate::service::{get_wishlist_service, WishlistService};

pub struct SimpleWishlistController {
    wishlist_service: Arc<dyn WishlistService>,
    cache: Arc<Mutex<LruCache<String, Wishlist>>>,
}

impl SimpleWishlistController {
    pub fn new() -> Result<Self> {
        Ok(Self {
            wishlist_service: get_wishlist_service()
                .ok_or(Error::ServiceUninitialized("wishlist_service".to_string()))?,
            cache: Arc::new(Mutex::new(LruCache::with_expiry_duration(
                Duration::from_secs(60),
            ))),
        })
    }
}

impl WishlistController for SimpleWishlistController {
    fn get_last_wishlist(&self) -> Result<Wishlist> {
        let mut guard = self.cache.lock().map_err(|_| {
            Error::PoisonedMutex("SimpleWishlistController::get_last_wishlist".to_string())
        })?;
        match (*guard).get("get-last-wishlist") {
            Some(w) => {
                debug!("Retrieving last wishlist from cache");
                Ok(w.clone())
            }
            None => {
                debug!("Retrieving last wishlist from fallback");
                let wishlist = self.wishlist_service.get_last_wishlist()?;
                (*guard).insert("get-last-wishlist".to_string(), wishlist.clone());
                Ok(wishlist)
            }
        }
    }
}
