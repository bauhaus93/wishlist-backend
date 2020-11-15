use mongodb::bson::oid::ObjectId;
use std::sync::{Arc, Mutex};

use crate::cache::Cache;
use crate::model::Source;
use crate::persistence::{get_source_dao, Error, Result, SourceDao};

#[derive(Clone)]
pub struct CachedSourceDao {
    cache: Arc<Mutex<Cache<Source>>>,
    fallback: Arc<dyn SourceDao>,
}

impl CachedSourceDao {
    pub fn new(fallback: Box<dyn SourceDao>, ttl_seconds: u64) -> Result<Self> {
        info!("Creating new cached source dao");
        Ok(Self {
            cache: Arc::new(Mutex::new(Cache::with_ttl(ttl_seconds))),
            fallback: Arc::from(fallback),
        })
    }
}

impl SourceDao for CachedSourceDao {
    fn get_source_by_id(&self, id: &ObjectId) -> Result<Source> {
        match self.cache.lock() {
            Ok(mut guard) => (*guard).get(&format!("id_{}", id.to_hex()), || {
                self.fallback.get_source_by_id(id)
            }),
            Err(_) => Err(Error::PoisonedMutex),
        }
    }
}
