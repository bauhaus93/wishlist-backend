use crate::persistence::{Error, ProductDao, Result, SourceDao, WishlistDao};

use super::{MongoProductDao, MongoSourceDao, MongoWishlistDao};

lazy_static! {
    static ref PRODUCT_DAO: Option<MongoProductDao> = MongoProductDao::new().ok();
    static ref WISHLIST_DAO: Option<MongoWishlistDao> = MongoWishlistDao::new().ok();
    static ref SOURCE_DAO: Option<MongoSourceDao> = MongoSourceDao::new().ok();
}

pub fn get_product_dao() -> Option<Box<MongoProductDao>> {
    PRODUCT_DAO.as_ref().map(|dao| Box::new(dao.clone()))
}

pub fn get_wishlist_dao() -> Option<Box<MongoWishlistDao>> {
    WISHLIST_DAO.as_ref().map(|dao| Box::new(dao.clone()))
}

pub fn get_source_dao() -> Option<Box<MongoSourceDao>> {
    SOURCE_DAO.as_ref().map(|dao| Box::new(dao.clone()))
}
