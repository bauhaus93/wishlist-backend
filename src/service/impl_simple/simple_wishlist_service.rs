use mongodb::bson::oid::ObjectId;
use std::collections::BTreeMap;

use crate::model::{Product, Source, Wishlist};
use crate::persistence::{
    get_cached_source_dao, get_product_dao, get_source_dao, get_wishlist_dao, ProductDao,
    SourceDao, WishlistDao,
};
use crate::service::{Error, Result, WishlistService};

pub struct SimpleWishlistService {
    wishlist_dao: Box<dyn WishlistDao>,
    product_dao: Box<dyn ProductDao>,
    source_dao: Box<dyn SourceDao>,
}

impl SimpleWishlistService {
    pub fn new() -> Result<SimpleWishlistService> {
        Ok(Self {
            wishlist_dao: get_wishlist_dao()
                .ok_or(Error::DaoUninitialized("wishlist_dao".to_string()))?,
            product_dao: get_product_dao()
                .ok_or(Error::DaoUninitialized("product_dao".to_string()))?,
            source_dao: get_cached_source_dao()
                .ok_or(Error::DaoUninitialized("source_dao".to_string()))?,
        })
    }
}

impl WishlistService for SimpleWishlistService {
    fn get_last_wishlist(&self) -> Result<Wishlist> {
        let mut wishlist = self.wishlist_dao.get_last_wishlist()?;
        let mut products = self.product_dao.get_products_for_wishlist(&wishlist)?;

        for product in products.iter_mut() {
            let source_id = product
                .get_source_id()
                .ok_or(Error::FieldNotLoaded("product", "source_id"))?;
            let source = self.source_dao.get_source_by_id(&source_id)?;
            product.set_source(source.clone());
        }

        wishlist.set_products(products);

        Ok(wishlist)
    }
}
