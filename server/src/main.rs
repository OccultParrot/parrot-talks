use axum::{
    Json, Router,
    extract::{Query, Request, State},
    http::{ header, Method },
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::{get, post},
};
use chrono::{DateTime, Utc};
use colored::Colorize;
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use subtle::ConstantTimeEq;
use tower_http::cors::{Any, CorsLayer};

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

#[derive(Serialize, Clone, sqlx::FromRow)]
struct Post {
    title: String,
    content: String,
    created_at: DateTime<Utc>,
    id: i32,
}

#[derive(Deserialize, Serialize, Clone)]
struct CreatePost {
    title: String,
    content: String,
}

#[derive(Deserialize)]
struct Pagination {
    limit: Option<i64>,
    page: Option<i64>,
}

async fn logging_middleware(req: Request, next: Next) -> Response {
    let method_color = match req.method() {
        &hyper::Method::GET => "blue",
        &hyper::Method::POST => "green",
        &hyper::Method::PUT => "yellow",
        &hyper::Method::DELETE => "red",
        _ => "white",
    };

    println!(
        "{} Request received: {} {}",
        "->".cyan(),
        req.method().to_string().color(method_color),
        req.uri()
    );

    let res = next.run(req).await;

    let res_color = match res.status() {
        StatusCode::OK => "green",
        StatusCode::CREATED => "green",
        StatusCode::BAD_REQUEST => "yellow",
        StatusCode::UNAUTHORIZED => "red",
        StatusCode::INTERNAL_SERVER_ERROR => "red",
        _ => "white",
    };

    println!(
        "{} Response status: {}",
        "<-".cyan(),
        res.status().to_string().color(res_color)
    );

    res
}

async fn auth_middleware(State(state): State<AppState>, req: Request, next: Next) -> Response {
    let authorized = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .is_some_and(|token| token.as_bytes().ct_eq(state.secret_key.as_bytes()).into());

    if !authorized {
        return StatusCode::UNAUTHORIZED.into_response();
    }

    println!(
        "   {}",
        "Authorization successful for POST request.".green()
    );
    next.run(req).await
}

async fn root() -> &'static str {
    "Hello from my blog!"
}

async fn get_posts(
    State(state): State<AppState>,
    Query(pagination): Query<Pagination>,
) -> Result<Json<Vec<Post>>, StatusCode> {
    let limit = pagination.limit.unwrap_or(10).clamp(1, 100);
    let page = pagination.page.unwrap_or(1).max(1);
    let offset = (page - 1) * limit;

    let posts = sqlx::query_as::<_, Post>(
        "SELECT title, content, created_at, id FROM posts ORDER BY created_at DESC LIMIT $1 OFFSET $2",
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(&state.db)
    .await
    .map_err(|e| {
        eprintln!("get_posts failed: {e}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(posts))
}

async fn post_posts(
    State(state): State<AppState>,
    Json(payload): Json<CreatePost>,
) -> Result<(StatusCode, Json<Post>), StatusCode> {
    let post = sqlx::query_as::<_, Post>(
        "INSERT INTO posts (title, content) VALUES ($1, $2) RETURNING title, content, created_at, id",
    )
    .bind(&payload.title)
    .bind(&payload.content)
    .fetch_one(&state.db)
    .await
    .map_err(|e| {
        eprintln!("post_posts failed: {e}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    Ok((StatusCode::CREATED, Json(post)))
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

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET]);

    let app = Router::new()
        .route("/", get(root))
        .route(
            "/posts",
            get(get_posts).merge(post(post_posts).route_layer(middleware::from_fn_with_state(
                state.clone(),
                auth_middleware,
            ))),
        )
        .layer(cors)
        .layer(middleware::from_fn(logging_middleware))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Server running on {}", listener.local_addr().unwrap());
    axum::serve(listener, app)
        .with_graceful_shutdown(graceful_shutdown_signal())
        .await
        .unwrap();
}
