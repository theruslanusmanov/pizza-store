use crate::domain::products::models::ProductsDto;
use async_trait::async_trait;
use mockall::automock;
use sqlx::FromRow;

#[automock]
#[async_trait]
pub trait ProductsRepository {
    async fn get_products(&self) -> anyhow::Result<Vec<ProductsDto>>;
}

#[derive(FromRow)]
pub struct GetProductQuery {
    pub id: i64,
}