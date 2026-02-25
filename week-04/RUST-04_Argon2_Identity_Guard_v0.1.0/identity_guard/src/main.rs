use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::atomic::{AtomicU64, Ordering},
};
use tokio::sync::RwLock;
use tracing::{info, warn};

mod core;
use core::hasher::{benchmark_and_tune, hash_password, params_from_env, verify_password};
use core::models::User;

#[derive(Default)]
struct AppState {
    users: RwLock<HashMap<String, User>>,
    next_id: AtomicU64,
}

#[derive(Debug, Deserialize)]
struct RegisterRequest {
    username: String,
    password: String,
}

#[derive(Debug, Serialize)]
struct RegisterResponse {
    id: u64,
    username: String,
    password_hash: String,
}

#[derive(Debug, Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Debug, Serialize)]
struct LoginResponse {
    ok: bool,
}

#[derive(Debug, Serialize)]
struct HealthResponse {
    status: &'static str,
}

#[derive(Debug, Serialize)]
struct BenchmarkResponse {
    memory_mib: u32,
    iterations: u32,
    parallelism: u32,
    elapsed_ms: u128,
    meets_100ms: bool,
}

/// CLI:
/// - serve (default) -> API
/// - hash `password`
/// - verify `hash` `password`
/// - benchmark
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string()))
        .init();

    let mut args = std::env::args().skip(1);
    match args.next().as_deref() {
        None | Some("serve") => serve().await,
        Some("hash") => {
            let password = args.next().unwrap_or_default();
            if password.is_empty() {
                eprintln!("usage: identity_guard hash <password>");
                std::process::exit(2);
            }
            match hash_password(&password) {
                Ok(h) => println!("{h}"),
                Err(e) => {
                    eprintln!("hash failed: {e:?}");
                    std::process::exit(1);
                }
            }
        }
        Some("verify") => {
            let hash = args.next().unwrap_or_default();
            let password = args.next().unwrap_or_default();
            if hash.is_empty() || password.is_empty() {
                eprintln!("usage: identity_guard verify <hash> <password>");
                std::process::exit(2);
            }
            println!("{}", verify_password(&hash, &password));
        }
        Some("benchmark") => {
            let base = params_from_env();
            match benchmark_and_tune(base) {
                Ok((p, d)) => {
                    let out = BenchmarkResponse {
                        memory_mib: p.memory_mib,
                        iterations: p.iterations,
                        parallelism: p.parallelism,
                        elapsed_ms: d.as_millis(),
                        meets_100ms: d.as_millis() >= 100,
                    };
                    println!("{}", serde_json::to_string(&out).unwrap());
                }
                Err(e) => {
                    eprintln!("benchmark failed: {e:?}");
                    std::process::exit(1);
                }
            }
        }
        Some(other) => {
            eprintln!("unknown command: {other}");
            eprintln!("commands: serve | hash | verify | benchmark");
            std::process::exit(2);
        }
    }
}

async fn serve() {
    let state = std::sync::Arc::new(AppState::default());

    let app = Router::new()
        .route("/health", get(health))
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/benchmark", get(benchmark_http))
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    info!("listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn health() -> impl IntoResponse {
    (StatusCode::OK, Json(HealthResponse { status: "ok" }))
}

async fn register(
    State(state): State<std::sync::Arc<AppState>>,
    Json(req): Json<RegisterRequest>,
) -> impl IntoResponse {
    if req.username.trim().is_empty() || req.password.is_empty() {
        return (StatusCode::BAD_REQUEST, "username/password required").into_response();
    }

    let hashed = match hash_password(&req.password) {
        Ok(v) => v,
        Err(e) => {
            warn!("hash_password failed: {e:?}");
            return (StatusCode::INTERNAL_SERVER_ERROR, "hash failed").into_response();
        }
    };

    let id = state.next_id.fetch_add(1, Ordering::Relaxed) + 1;
    let user = User {
        id,
        username: req.username.clone(),
        password_hash: hashed.clone(),
    };

    let mut users = state.users.write().await;
    if users.contains_key(&req.username) {
        return (StatusCode::CONFLICT, "username already exists").into_response();
    }
    users.insert(req.username.clone(), user);

    (
        StatusCode::CREATED,
        Json(RegisterResponse {
            id,
            username: req.username,
            password_hash: hashed,
        }),
    )
        .into_response()
}

async fn login(
    State(state): State<std::sync::Arc<AppState>>,
    Json(req): Json<LoginRequest>,
) -> impl IntoResponse {
    let users = state.users.read().await;
    let user = match users.get(&req.username) {
        Some(u) => u,
        None => {
            return (StatusCode::UNAUTHORIZED, Json(LoginResponse { ok: false })).into_response();
        }
    };

    let ok = verify_password(&user.password_hash, &req.password);

    if ok {
        (StatusCode::OK, Json(LoginResponse { ok: true })).into_response()
    } else {
        (StatusCode::UNAUTHORIZED, Json(LoginResponse { ok: false })).into_response()
    }
}

async fn benchmark_http() -> impl IntoResponse {
    let base = params_from_env();
    match benchmark_and_tune(base) {
        Ok((p, d)) => (
            StatusCode::OK,
            Json(BenchmarkResponse {
                memory_mib: p.memory_mib,
                iterations: p.iterations,
                parallelism: p.parallelism,
                elapsed_ms: d.as_millis(),
                meets_100ms: d.as_millis() >= 100,
            }),
        )
            .into_response(),
        Err(e) => {
            warn!("benchmark failed: {e:?}");
            (StatusCode::INTERNAL_SERVER_ERROR, "benchmark failed").into_response()
        }
    }
}
