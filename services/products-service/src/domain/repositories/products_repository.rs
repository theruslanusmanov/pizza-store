use crate::core::products_repository::ProductsRepository;
use crate::domain::connection_pool::PizzaConnectionPool;
use crate::domain::products::models::ProductsDto;
use async_trait::async_trait;
use sqlx::query_file_as;

#[derive(Clone)]
pub struct PostgresProductsRepository {
    pool: PizzaConnectionPool,
}

impl PostgresProductsRepository {
    pub fn new(pool: PizzaConnectionPool) -> Self { Self { pool } }
}

#[async_trait]
impl ProductsRepository for PostgresProductsRepository {
    async fn get_products(&self) -> anyhow::Result<Vec<ProductsDto>> {
        query_file_as!(
            GetProductQuery,
            "queries/get_articles.sql",
        )
            .fetch_all(&self.pool)
            .await
            .context("an unexpected error occured retrieving articles")
    }
}