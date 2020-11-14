use bson::document::{Document, ValueAccessError};
use serde::Serialize;

use super::Source;

#[derive(Serialize)]
pub struct Product {
    #[serde(skip)]
    id: Option<String>,
    name: Option<String>,
    price: Option<i32>,
    quantity: Option<i32>,
    stars: Option<i32>,
    url: Option<String>,
    url_img: Option<String>,
    #[serde(skip)]
    item_id: Option<String>,
    first_seen: Option<i32>,
    last_seen: Option<i32>,
    #[serde(skip)]
    source_id: Option<String>,
    source: Option<Source>,
}

impl From<&Document> for Product {
    fn from(doc: &Document) -> Self {
        Self {
            id: doc.get_object_id("_id").map(|e| e.to_hex()).ok(),
            name: doc.get_str("name").map(String::from).ok(),
            price: doc.get_i32("price").ok(),
            quantity: doc.get_i32("price").ok(),
            stars: doc.get_i32("stars").ok(),
            url: doc.get_str("url").map(String::from).ok(),
            url_img: doc.get_str("url_img").map(String::from).ok(),
            item_id: doc.get_str("item_id").map(String::from).ok(),
            first_seen: doc.get_i32("first_seen").ok(),
            last_seen: doc.get_i32("last_seen").ok(),
            source_id: doc.get_object_id("source").map(|e| e.to_hex()).ok(),
            source: None,
        }
    }
}
