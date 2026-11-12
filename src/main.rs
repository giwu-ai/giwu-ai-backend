mod agents;
mod handlers;
mod models;

use axum::{
    routing::post,
    Router,
};
use handlers::{compare_product_handler, verify_product_handler};
use models::AppState;
use rig::providers::gemini;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let api_key = std::env::var("GEMINI_API_KEY")
        .expect("GEMINI_API_KEY environment variable must be set");

    let client = gemini::Client::new(&api_key);
    let state = AppState { gemini_client: client };

    let app = Router::new()
        .route("/wp-json/thewall/v1/verify-product", post(verify_product_handler))
        .route("/wp-json/thewall/v1/compare-product", post(compare_product_handler))
        .layer(CorsLayer::permissive())
        .with_state(state);

    let addr = "0.0.0.0:8080";
    tracing::info!("🚀 GIWU AI Direct Modular Engine running on http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
