use mongodb::bson::oid::ObjectId;

use super::{Error, Result};
use crate::model::{Product, Source};

pub trait SourceDao: Send + Sync {
    fn get_source_by_id(&self, id: &ObjectId) -> Result<Source>;
    fn get_source_for_product(&self, product: &Product) -> Result<Source> {
        product
            .get_source_id()
            .ok_or(Error::FieldNotLoaded("product", "source_id"))
            .and_then(|sid| self.get_source_by_id(sid))
    }
}
