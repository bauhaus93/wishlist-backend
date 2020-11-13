use super::Result;
use crate::model::Product;

pub trait ProductDao: Send + Sync {
    fn get_products_by_id(&self, ids: &[String]) -> Result<Vec<Product>>;
    fn get_archived_products(&self, page: usize, per_page: usize) -> Result<Vec<Product>>;
}
