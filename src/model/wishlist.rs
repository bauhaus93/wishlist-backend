use bson::document::{Document, ValueAccessError};
use serde::Serialize;
use std::convert::TryFrom;

use super::Product;

#[derive(Serialize)]
pub struct Wishlist {
    #[serde(skip)]
    id: String,
    timestamp: i32,
    products: Vec<Product>,
}

impl TryFrom<Document> for Wishlist {
    type Error = ValueAccessError;

    fn try_from(doc: Document) -> Result<Self, Self::Error> {
        Ok(Self {
            id: doc.get_object_id("_id").map(|e| e.to_hex())?,
            timestamp: doc.get_i32("timestamp")?,
            products: doc.get_array("products").map(|bson| {
                bson.iter()
                    .filter_map(|e| {
                        e.as_document()
                            .ok_or(Self::Error::UnexpectedType)
                            .and_then(Product::try_from)
                            .map_or(None, Option::Some)
                    })
                    .collect()
            })?,
        })
    }
}
