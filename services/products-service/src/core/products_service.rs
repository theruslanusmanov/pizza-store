use async_trait::async_trait;
use mockall::automock;
use crate::domain::products::models::ProductsDto;

#[automock]
#[async_trait]
pub trait ProductsService {
    async fn get_products() -> Result<Vec<ProductsDto>, String>;
}