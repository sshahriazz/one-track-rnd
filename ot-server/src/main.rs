#![deny(unsafe_code)]
use axum::Router;
use chrono::Local;
use config::env::EnvironmentVariables;
use routes::{project_route::project_routes, section_route::section_routes};

use sea_orm::{DatabaseConnection, DbErr};
use std::{error::Error, panic, sync::Arc, time::Duration};
use tokio::time::sleep;
use tower_http::trace::TraceLayer;
use tracing::{error, info, warn};
use tracing_subscriber::{fmt::time::time, EnvFilter};

mod config;
mod dtos;
mod entities;
mod handlers;
mod integrations;
mod migrator;
mod repository;
mod routes;
mod services;
mod utils;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<DatabaseConnection>,
    pub env: Arc<EnvironmentVariables>,
    pub client: reqwest::Client,
    pub is_standalone: bool,
}

impl AppState {
    pub async fn from_env() -> anyhow::Result<Self> {
        let env = EnvironmentVariables::from_env()?;

        // Implement connection retry logic
        let mut retry_count = 0;
        let max_retries = 5;
        let mut db = None;

        while retry_count < max_retries {
            match config::database::initialize_database(&env).await {
                Ok(connection) => {
                    db = Some(connection);
                    break;
                }
                Err(e) => {
                    retry_count += 1;
                    error!("Database connection attempt {} failed: {}", retry_count, e);
                    if retry_count < max_retries {
                        let backoff = Duration::from_secs(2u64.pow(retry_count as u32));
                        warn!("Retrying connection in {} seconds...", backoff.as_secs());
                        sleep(backoff).await;
                    }
                }
            }
        }

        let db = db.ok_or_else(|| {
            anyhow::anyhow!(
                "Failed to connect to database after {} attempts",
                max_retries
            )
        })?;
        info!("Successfully connected to database");

        Ok(Self {
            db: Arc::new(db),
            env: Arc::new(env),
            client: reqwest::Client::new(),
            is_standalone: true,
        })
    }

    // Helper method to check database connection health
    pub async fn check_db_health(&self) -> bool {
        match self.db.ping().await {
            Ok(_) => true,
            Err(e) => {
                error!("Database health check failed: {}", e);
                false
            }
        }
    }
}

async fn shutdown_signal(rx: tokio::sync::oneshot::Receiver<()>) {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("Failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    let rx = async {
        rx.await.ok();
    };

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
        _ = rx => {},
    }

    println!("[Backend] Shutdown signal received, starting graceful shutdown...");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Set up panic hook for crash reporting
    panic::set_hook(Box::new(|panic_info| {
        error!("Server panic occurred: {}", panic_info);
        // You could add additional crash reporting here (e.g., to a monitoring service)
    }));

    // Initialize logging first
    // Initialize more verbose logging
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| {
            EnvFilter::new("ot_server=debug,tower_http=debug,axum::rejection=trace,sea_orm=debug")
        }))
        .with_timer(time())
        .with_thread_names(true)
        .with_target(true)
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_level(true)
        .init();

    info!("Starting server initialization");

    // Initialize application state with retry logic
    let state = AppState::from_env().await?;

    // Database is already initialized in AppState

    let api_routes = Router::new()
        .nest("/project", project_routes())
        .nest("/section", section_routes())
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(
                    tower_http::trace::DefaultMakeSpan::new().level(tracing::Level::INFO),
                )
                .on_request(tower_http::trace::DefaultOnRequest::new().level(tracing::Level::INFO))
                .on_response(
                    tower_http::trace::DefaultOnResponse::new().level(tracing::Level::INFO),
                )
                .on_failure(
                    tower_http::trace::DefaultOnFailure::new().level(tracing::Level::ERROR),
                ),
        );

    let app = Router::new()
        .nest("/api", api_routes)
        .with_state(state.clone());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    info!("Server listening on port 3000");

    // Set up graceful shutdown
    let (tx, rx) = tokio::sync::oneshot::channel::<()>();

    // Clone state for health check
    let state_clone = state.clone();

    // Spawn health check task
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(30));
        loop {
            interval.tick().await;
            if !state_clone.check_db_health().await {
                error!("Database health check failed. Initiating graceful shutdown...");
                let _ = tx.send(());
                break;
            }
        }
    });

    let server =
        axum::serve(listener, app.into_make_service()).with_graceful_shutdown(shutdown_signal(rx));

    info!("Server initialization complete. Press Ctrl+C to stop");

    match server.await {
        Ok(_) => info!("Server shutdown completed successfully"),
        Err(e) => error!("Server error: {}", e),
    }
    Ok(())
}
