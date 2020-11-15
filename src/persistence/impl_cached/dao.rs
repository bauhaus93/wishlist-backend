use crate::persistence::{get_source_dao, Error, ProductDao, Result, SourceDao, WishlistDao};

use super::CachedSourceDao;

lazy_static! {
    static ref SOURCE_DAO: Option<CachedSourceDao> =
        get_source_dao().and_then(|dao| CachedSourceDao::new(dao, 60).ok());
}

pub fn get_cached_source_dao() -> Option<Box<CachedSourceDao>> {
    SOURCE_DAO.as_ref().map(|dao| Box::new(dao.clone()))
}
