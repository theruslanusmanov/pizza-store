use crate::core::products_service::ProductsService;
use crate::di::service_register::ServiceRegister;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Extension, Json, Router};
use std::sync::Arc;

pub struct ProductsRouter;

impl ProductsRouter {
    pub fn init(service_register: ServiceRegister) -> Router {
        Router::new()
            .route("/products", get(get_products))
            .layer(Extension(service_register.products_service))
    }
}

pub async fn get_products(
    Extension(products_service): Extension<Arc<dyn ProductsService + Send + Sync>>,
) -> impl IntoResponse {
    let products = products_service.get_products().await.unwrap();

    Json(products).into_response()
}
