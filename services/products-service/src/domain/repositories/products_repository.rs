use crate::connection_pool::PizzaConnectionPool;
use crate::core::products_repository::ProductsRepository;
use crate::domain::products::models::ProductsDto;
use async_trait::async_trait;
use sqlx::{FromRow, query};

#[derive(FromRow)]
struct GetProductQuery {
    pub id: i64,
}

#[derive(Clone)]
pub struct PostgresProductsRepository {
    pool: PizzaConnectionPool,
}

impl PostgresProductsRepository {
    pub fn new(pool: PizzaConnectionPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ProductsRepository for PostgresProductsRepository {
    async fn get_products(&self) -> anyhow::Result<Vec<ProductsDto>> {
        let product = ProductsDto {
            id: 0,
        };
        Ok(vec![product])
       /* query!("SELECT * FROM products as products")
            .fetch_all(&self.pool)
            .await
            .context("an unexpected error occured retrieving articles")*/
    }
}
