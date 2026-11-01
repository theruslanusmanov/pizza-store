use crate::config::AppConfig;
use crate::connection_pool::PizzaConnectionManager;
use crate::router::PizzaApplicationController;
use anyhow::Context;
use clap::Parser;
use di::service_register::ServiceRegister;
use std::sync::Arc;
use tracing::info;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

mod api;
mod di;
mod core;
mod domain;
mod config;
mod connection_pool;
mod router;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    let config = Arc::new(AppConfig::parse());

    tracing_subscriber::registry()
        .with(EnvFilter::new(&config.rust_log))
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("environment loaded and configuration parsed, initializing Postgres connection and running migrations...");
    let pg_pool = PizzaConnectionManager::new_pool(&config.database_url, config.run_migrations)
        .await
        .expect("could not initialize the database connection pool");

    let port = config.port;
    let service_register = ServiceRegister::new(pg_pool/*, config.clone()*/);

    // if config.seed {
    //     info!("seeding enabled, creating test data...");
    //     PizzaSeedService::new(service_register.clone())
    //         .seed()
    //         .await
    //         .expect("unexpected error occurred while seeding application data");
    // }

    info!("migrations successfully ran, initializing axum server...");
    PizzaApplicationController::serve(port, &config.cors_origin, service_register)
        .await
        .context("could not initialize application routes")?;

    Ok(())
}
