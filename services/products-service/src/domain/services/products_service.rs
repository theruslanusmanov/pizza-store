use std::sync::Arc;
use anyhow::Error;
use async_trait::async_trait;
use crate::core::products_repository::ProductsRepository;
use crate::core::products_service::ProductsService;
use crate::domain::products::models::ProductsDto;

pub struct PizzaProductsService {
    products_repository: Arc<dyn ProductsRepository + Send + Sync>,
}

impl PizzaProductsService {
    pub fn new(products_repository: Arc<dyn ProductsRepository + Send + Sync>) -> Self {
        Self {
            products_repository
        }
    }
}

#[async_trait]
impl ProductsService for PizzaProductsService {
    async fn get_products(&self) -> Result<Vec<ProductsDto>, Error> {
        let products = self
            .products_repository
            .get_products()
            .await?;

        Ok(products.into_iter().map(ProductsDto::from).collect())
    }
}