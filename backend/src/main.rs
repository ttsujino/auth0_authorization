use axum::{
    extract::Extension,
    routing::{get, post, delete},
    Router,
};
use repositories::{PostRepository, PostRepositoryForDb};
use tracing_subscriber;
use std::{env, sync::Arc};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;

use handlers::{create_post, get_all_posts, get_target_user_posts, get_post, delete_post, return_uid};
use tower_http::cors::{Any, CorsLayer};
use tower_http::set_header::SetResponseHeaderLayer;
use axum::http::{HeaderName, HeaderValue};

#[tokio::main]
async fn main() {
    let log_level = env::var("RUST_LOG").unwrap_or("debug".to_string());
    env::set_var("RUST_LOG", log_level);
    tracing_subscriber::fmt::init();
    dotenv().ok();

    let app = create_app()

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

}

fn create_app<T: PostRepository>(repository: T) -> Router {
    Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/check_token", post(return_uid))
}
