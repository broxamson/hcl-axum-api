use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::routes::create_routes;

mod jenkins;
mod routes;
mod git_func;

pub async fn web() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "netreoxide=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("initializing router and assets");

    let app = create_routes();

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:4000".parse().unwrap())
        .serve(app.await.into_make_service())
        .await
        .unwrap();
}
