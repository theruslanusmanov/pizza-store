use crate::config::AppConfig;
use crate::connection_pool::PizzaConnectionPool;
use crate::core::products_service::ProductsService;
use std::sync::Arc;
use crate::core::products_repository::ProductsRepository;
use crate::domain::repositories::products_repository::PostgresProductsRepository;
use crate::domain::services::products_service::PizzaProductsService;

#[derive(Clone)]
pub struct ServiceRegister {
    pub products_service: Arc<dyn ProductsService + Send + Sync>,
}

impl ServiceRegister {
    pub fn new(pool: PizzaConnectionPool, config: Arc<AppConfig>) -> Self {
        let products_repository = Arc::new(PostgresProductsRepository::new(pool.clone()))
            as Arc<dyn ProductsRepository + Send + Sync>;
        let products_service = Arc::new(PizzaProductsService::new(products_repository.clone()))
            as Arc<dyn ProductsService + Send + Sync>;

        ServiceRegister { products_service }
    }
}
