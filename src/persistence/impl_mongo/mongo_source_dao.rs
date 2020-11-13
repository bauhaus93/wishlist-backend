use mongodb::{
    bson::{doc, oid::ObjectId, Bson},
    options::FindOptions,
    sync::Client,
};
use std::convert::TryFrom;

use super::get_mongo_client;
use crate::model::Source;
use crate::persistence::{Error, Result, SourceDao};

pub struct MongoSourceDao {
    client: Client,
}

impl MongoSourceDao {
    pub fn new() -> Result<Self> {
        Ok(Self {
            client: get_mongo_client()?,
        })
    }
}
impl SourceDao for MongoSourceDao {
    fn get_source_by_id(&self, id: &str) -> Result<Source> {
        let coll = self.client.database("wishlist").collection("source");

        coll.find_one(Some(doc! {"_id": ObjectId::with_string(id)?}), None)
            .map_err(Error::from)
            .and_then(|r| r.ok_or(Error::EmptyResult))
            .and_then(|s| Source::try_from(&s).map_err(Error::from))
    }
}
