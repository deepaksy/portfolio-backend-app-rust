
use axum::{
    http::{
        header::{AUTHORIZATION, CONTENT_TYPE},
        HeaderValue, Method,
    },
    routing::{get, post},
    Router,
};

use dotenvy::dotenv;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};

use std::str::FromStr;
use std::{env, sync::LazyLock};

use tower_http::{
    compression::CompressionLayer,
    cors::CorsLayer,
    cors::Any
};

use portfolio_backend_app::{
    models::feedbacks::AppState,
    routes::health::get_health,
};

use portfolio_backend_app::routes::feedbacks::{
    create_feedback,
    delete_feedback,
    get_all_feedback,
    get_feedback_by_id,
    update_feedback,
};

use tracing_subscriber::{
    layer::SubscriberExt,
    util::SubscriberInitExt,
};

static PORT: LazyLock<String> = LazyLock::new(|| {
    env::var("PORT").unwrap_or_else(|_| "8000".to_string())
});

static HOST: LazyLock<String> = LazyLock::new(|| {
    env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string())
});

#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL must be specified");

    let options = PgConnectOptions::from_str(&database_url)
        .expect("Failed to parse DATABASE_URL")
        .statement_cache_capacity(0);

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await
        .expect("Failed to connect to Supabase PostgreSQL");

    use sqlx::Executor;

    pool.execute(
        r#"
        CREATE TABLE IF NOT EXISTS feedback (
            id SERIAL PRIMARY KEY,
            name VARCHAR(255) NOT NULL,
            email VARCHAR(255) UNIQUE,
            phone VARCHAR(50),
            message TEXT NOT NULL
        );
        "#,
    )
    .await
    .expect("Failed to verify/initialize the feedback table structure");

    let shared_state = AppState { pool };

    // CORS configuration
    let cors = CorsLayer::new()
        // .allow_origin(Any)
        .allow_origin([
            HeaderValue::from_static("http://localhost:4321"),
            HeaderValue::from_static("https://iamengineer.in"),
            HeaderValue::from_static("https://www.iamengineer.in"),
            HeaderValue::from_static("*")
        ])
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers([
            CONTENT_TYPE,
            AUTHORIZATION,
        ])
        .allow_credentials(true);

    let app = Router::new()
        .route("/api/feedback", post(create_feedback).get(get_all_feedback))
        .route(
            "/api/feedback/{id}",
            get(get_feedback_by_id)
                .put(update_feedback)
                .delete(delete_feedback),
        )
        .route("/api/health", get(get_health))
        .layer(cors)
        .layer(CompressionLayer::new().gzip(true))
        .with_state(shared_state);

    let address_str = format!("{}:{}", *HOST, *PORT);

    let listener = tokio::net::TcpListener::bind(&address_str)
        .await
        .unwrap();

    println!("Server running on http://{}", address_str);
    tracing::info!("🚀 Server running on {}", address_str);

    axum::serve(listener, app).await.unwrap();
}
