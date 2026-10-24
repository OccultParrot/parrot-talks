use axum::{
    Json, Router,
    extract::Request,
    middleware::{self, Next},
    response::Response,
    routing::get,
};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;

/*
Endpoints:
- GET / - Returns a greeting message to verify that the server is running.
- GET /posts - Returns blog posts.
    - Request Parameters:
        - Optional number of posts to return (default: 10).
        - Optional page number for pagination (default: 1).
- POST /posts - Creates a new blog post.
    - Request Body:
        - JSON object containing the post title and content.
        - Secret key for authentication (to prevent unauthorized post creation). (specified in the .env file)
*/

#[derive(Clone)]
struct AppState {
    db: sqlx::PgPool,
    secret_key: Arc<String>,
}

#[derive(Serialize, Clone)]
struct Post {
    title: String,
    content: String,
}

#[derive(Deserialize, Serialize, Clone)]
struct CreatePost {
    title: String,
    content: String,
    secret_key: String,
}

async fn logging_middleware(req: Request, next: Next) -> Response {
    println!("-> Request received: {} {}", req.method(), req.uri());
    let res = next.run(req).await;
    println!("<- Response status: {}", res.status());

    res
}

async fn root() -> &'static str {
    "Hello from Axum!"
}

async fn get_posts() -> (StatusCode, Json<Vec<Post>>) {
    return (StatusCode::OK, Json(vec![]));
}

async fn post_posts(Json(payload): Json<CreatePost>) -> (StatusCode, Json<Post>) {
    return (
        StatusCode::OK,
        Json(Post {
            title: payload.title,
            content: payload.content,
        }),
    );
}

async fn graceful_shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    // SIGTERM is what Docker, Kubernetes, and systemd send on shutdown
    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    println!("Shutdown signal received, finishing in-flight requests...");
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&std::env::var("DATABASE_URL").expect("DATABASE_URL must be set"))
        .await
        .expect("Failed to connect to database");

    let state = AppState {
        db: pool,
        secret_key: Arc::new(std::env::var("SECRET_KEY").expect("SECRET_KEY must be set")),
    };

    let app = Router::new()
        .route("/", get(root))
        .route("/posts", get(get_posts).post(post_posts))
        .with_state(state)
        .layer(middleware::from_fn(logging_middleware));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Server running on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).with_graceful_shutdown(graceful_shutdown_signal()).await.unwrap();
}
use axum::{
    Json, Router,
    extract::Request,
    middleware::{self, Next},
    response::Response,
    routing::get,
};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;

/*
Endpoints:
- GET / - Returns a greeting message to verify that the server is running.
- GET /posts - Returns blog posts.
    - Request Parameters:
        - Optional number of posts to return (default: 10).
        - Optional page number for pagination (default: 1).
- POST /posts - Creates a new blog post.
    - Request Body:
        - JSON object containing the post title and content.
        - Secret key for authentication (to prevent unauthorized post creation). (specified in the .env file)
*/

#[derive(Clone)]
struct AppState {
    db: sqlx::PgPool,
    secret_key: Arc<String>,
}

#[derive(Serialize, Clone)]
struct Post {
    title: String,
    content: String,
}

#[derive(Deserialize, Serialize, Clone)]
struct CreatePost {
    title: String,
    content: String,
    secret_key: String,
}

async fn logging_middleware(req: Request, next: Next) -> Response {
    println!("-> Request received: {} {}", req.method(), req.uri());
    let res = next.run(req).await;
    println!("<- Response status: {}", res.status());

    res
}

async fn root() -> &'static str {
    "Hello from Axum!"
}

async fn get_posts() -> (StatusCode, Json<Vec<Post>>) {
    return (StatusCode::OK, Json(vec![]));
}

async fn post_posts(Json(payload): Json<CreatePost>) -> (StatusCode, Json<Post>) {
    return (
        StatusCode::OK,
        Json(Post {
            title: payload.title,
            content: payload.content,
        }),
    );
}

async fn graceful_shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    // SIGTERM is what Docker, Kubernetes, and systemd send on shutdown
    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    println!("Shutdown signal received, finishing in-flight requests...");
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&std::env::var("DATABASE_URL").expect("DATABASE_URL must be set"))
        .await
        .expect("Failed to connect to database");

    let state = AppState {
        db: pool,
        secret_key: Arc::new(std::env::var("SECRET_KEY").expect("SECRET_KEY must be set")),
    };

    let app = Router::new()
        .route("/", get(root))
        .route("/posts", get(get_posts).post(post_posts))
        .with_state(state)
        .layer(middleware::from_fn(logging_middleware));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Server running on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).with_graceful_shutdown(graceful_shutdown_signal()).await.unwrap();
}
use axum::{
    Json, Router,
    extract::Request,
    middleware::{self, Next},
    response::Response,
    routing::get,
};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;

/*
Endpoints:
- GET / - Returns a greeting message to verify that the server is running.
- GET /posts - Returns blog posts.
    - Request Parameters:
        - Optional number of posts to return (default: 10).
        - Optional page number for pagination (default: 1).
- POST /posts - Creates a new blog post.
    - Request Body:
        - JSON object containing the post title and content.
        - Secret key for authentication (to prevent unauthorized post creation). (specified in the .env file)
*/

#[derive(Clone)]
struct AppState {
    db: sqlx::PgPool,
    secret_key: Arc<String>,
}

#[derive(Serialize, Clone)]
struct Post {
    title: String,
    content: String,
}

#[derive(Deserialize, Serialize, Clone)]
struct CreatePost {
    title: String,
    content: String,
    secret_key: String,
}

async fn logging_middleware(req: Request, next: Next) -> Response {
    println!("-> Request received: {} {}", req.method(), req.uri());
    let res = next.run(req).await;
    println!("<- Response status: {}", res.status());

    res
}

async fn root() -> &'static str {
    "Hello from Axum!"
}

async fn get_posts() -> (StatusCode, Json<Vec<Post>>) {
    return (StatusCode::OK, Json(vec![]));
}

async fn post_posts(Json(payload): Json<CreatePost>) -> (StatusCode, Json<Post>) {
    return (
        StatusCode::OK,
        Json(Post {
            title: payload.title,
            content: payload.content,
        }),
    );
}

async fn graceful_shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    // SIGTERM is what Docker, Kubernetes, and systemd send on shutdown
    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    println!("Shutdown signal received, finishing in-flight requests...");
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&std::env::var("DATABASE_URL").expect("DATABASE_URL must be set"))
        .await
        .expect("Failed to connect to database");

    let state = AppState {
        db: pool,
        secret_key: Arc::new(std::env::var("SECRET_KEY").expect("SECRET_KEY must be set")),
    };

    let app = Router::new()
        .route("/", get(root))
        .route("/posts", get(get_posts).post(post_posts))
        .with_state(state)
        .layer(middleware::from_fn(logging_middleware));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Server running on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).with_graceful_shutdown(graceful_shutdown_signal()).await.unwrap();
}

