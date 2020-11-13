use bson::document::{Document, ValueAccessError};
use serde::Serialize;
use std::convert::TryFrom;

#[derive(Serialize)]
pub struct Source {
    #[serde(skip)]
    pub id: String,
    pub name: String,
    pub url: String,
}

impl TryFrom<&Document> for Source {
    type Error = ValueAccessError;
    fn try_from(doc: &Document) -> Result<Self, Self::Error> {
        Ok(Self {
            id: doc.get_object_id("_id").map(|e| e.to_hex())?,
            name: doc.get_str("name").map(String::from)?,
            url: doc.get_str("url").map(String::from)?,
        })
    }
}
