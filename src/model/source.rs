use bson::document::Document;
use serde::Serialize;

#[derive(Serialize)]
pub struct Source {
    #[serde(skip)]
    pub id: Option<String>,
    pub name: Option<String>,
    pub url: Option<String>,
}

impl From<&Document> for Source {
    fn from(doc: &Document) -> Self {
        Self {
            id: doc.get_object_id("_id").map(|e| e.to_hex()).ok(),
            name: doc.get_str("name").map(String::from).ok(),
            url: doc.get_str("url").map(String::from).ok(),
        }
    }
}
