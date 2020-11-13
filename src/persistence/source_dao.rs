use super::Result;
use crate::model::Source;

pub trait SourceDao: Send + Sync {
    fn get_source_by_id(&self, id: &str) -> Result<Source>;
}
