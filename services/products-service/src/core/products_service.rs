use anyhow::Error;
use async_trait::async_trait;
use mockall::automock;
use crate::domain::products::models::ProductsDto;

#[automock]
#[async_trait]
pub trait ProductsService {
    async fn get_products(&self) -> Result<Vec<ProductsDto>, Error>;
}