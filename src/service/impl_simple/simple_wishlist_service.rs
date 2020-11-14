use crate::model::Wishlist;
use crate::persistence::{
    MongoProductDao, MongoSourceDao, MongoWishlistDao, ProductDao, SourceDao, WishlistDao,
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
            wishlist_dao: Box::new(MongoWishlistDao::new()?),
            product_dao: Box::new(MongoProductDao::new()?),
            source_dao: Box::new(MongoSourceDao::new()?),
        })
    }
}

impl WishlistService for SimpleWishlistService {
    fn get_last_wishlist(&self) -> Result<Wishlist> {
        let mut wishlist = self.wishlist_dao.get_last_wishlist()?;

        let products = wishlist
            .get_product_ids()
            .ok_or(Error::FieldNotLoaded("wishlist", "product_ids"))
            .and_then(|ids| {
                self.product_dao
                    .get_products_by_id(ids)
                    .map_err(Error::from)
            })?;

        wishlist.set_products(products);

        Ok(wishlist)
    }
}
