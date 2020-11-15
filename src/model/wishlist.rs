use mongodb::bson::{document::Document, oid::ObjectId};
use serde::Serialize;
use std::iter::Iterator;

use super::Product;

#[derive(Serialize)]
pub struct Wishlist {
    #[serde(skip)]
    id: Option<ObjectId>,
    timestamp: Option<i32>,
    #[serde(skip)]
    product_ids: Option<Vec<ObjectId>>,
    products: Option<Vec<Product>>,
}

impl Wishlist {
    pub fn get_product_ids(&self) -> Option<&[ObjectId]> {
        self.product_ids.as_ref().map(|e| e.as_slice())
    }
    pub fn set_products(&mut self, products: Vec<Product>) {
        self.products = Some(products);
    }
}

impl From<&Document> for Wishlist {
    fn from(doc: &Document) -> Self {
        Self {
            id: doc.get_object_id("_id").map(|id| id.clone()).ok(),
            timestamp: doc.get_i32("timestamp").ok(),
            product_ids: doc
                .get_array("products")
                .map(|list| {
                    list.into_iter()
                        .filter_map(|e| e.as_object_id().map(|id| id.clone()))
                        .collect()
                })
                .ok(),
            products: None,
        }
    }
}

/*
products: doc.get_array("products").map(|bson| {
               bson.iter()
                   .filter_map(|e| {
                       e.as_document()
                           .ok_or(Self::Error::UnexpectedType)
                           .and_then(Product::try_from)
                           .map_or(None, Option::Some)
                   })
                   .collect()
           })?,*/
