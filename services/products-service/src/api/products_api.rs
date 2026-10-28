use axum::{Json, Router};
use axum::routing::get;
use std::error::Error;
use crate::core::products_service::ProductsService;
use crate::di::service_register::ServiceRegister;
use crate::domain::products::models::ProductsDto;

pub struct ProductsApi;

impl ProductsApi {
    pub fn init(service_register: ServiceRegister) -> Router {
        Router::new().route("/products", get(get_products))
    }
}

pub async fn get_products() -> Result<Json<Vec<ProductsDto>>, String> {
    let products = ProductsService::get_products().await;

    Ok(Json(products?))
}
