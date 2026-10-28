use std::sync::Arc;
use crate::core::products_service::ProductsService;

#[derive(Clone)]
pub struct ServiceRegister {
    pub products_service: Arc<dyn ProductsService + Send + Sync>
}