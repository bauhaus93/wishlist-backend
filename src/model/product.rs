use bson::document::{Document, ValueAccessError};
use serde::Serialize;
use std::convert::TryFrom;

use super::Source;

#[derive(Serialize)]
pub struct Product {
    #[serde(skip)]
    id: String,
    name: String,
    price: i32,
    quantity: i32,
    stars: i32,
    url: String,
    url_img: String,
    #[serde(skip)]
    item_id: String,
    first_seen: i32,
    last_seen: i32,
    source: Source,
}

impl TryFrom<&Document> for Product {
    type Error = ValueAccessError;

    fn try_from(doc: &Document) -> Result<Self, Self::Error> {
        Ok(Self {
            id: doc.get_object_id("_id").map(|e| e.to_hex())?,
            name: doc.get_str("name").map(String::from)?,
            price: doc.get_i32("price")?,
            quantity: doc.get_i32("price")?,
            stars: doc.get_i32("stars")?,
            url: doc.get_str("url").map(String::from)?,
            url_img: doc.get_str("url_img").map(String::from)?,
            item_id: doc.get_str("item_id").map(String::from)?,
            first_seen: doc.get_i32("first_seen")?,
            last_seen: doc.get_i32("last_seen")?,
            source: doc.get_document("source").and_then(Source::try_from)?,
        })
    }
}
