use mongodb::{
    bson::{doc, oid::ObjectId, Bson},
    options::FindOptions,
    sync::Client,
};

use super::get_mongo_client;
use crate::model::Product;
use crate::persistence::{Error, ProductDao, Result};

pub struct MongoProductDao {
    client: Client,
}

impl MongoProductDao {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            client: get_mongo_client()?,
        })
    }
}

impl ProductDao for MongoProductDao {
    fn get_products_by_id(&self, product_ids: &[String]) -> Result<Vec<Product>> {
        let coll = self.client.database("wishlist").collection("product");
        Ok(Vec::new())
    }

    fn get_archived_products(&self, page: usize, per_page: usize) -> Result<Vec<Product>> {
        let coll = self.client.database("wishlist").collection("product");
        Ok(Vec::new())
    }
}
