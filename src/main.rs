use clap::Parser;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::signal;

use rust_template::core;
use rust_template::features::user::{repository::SqlUserRepository, service::UserService};
use rust_template::{create_app, AppState};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(long)]
    health_probe: bool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // 1. Telemetry
    core::telemetry::init_telemetry();

    // 2. Configuration
    let settings = core::config::Settings::load()?;

    if cli.health_probe {
        let url = format!("http://localhost:{}/healthz", settings.server_port);
        match reqwest::get(&url).await {
            Ok(resp) if resp.status() == reqwest::StatusCode::OK => {
                std::process::exit(0);
            }
            _ => {
                std::process::exit(1);
            }
        }
    }

    // Initialize core components

    let db_pool = Arc::new(core::db::DbPool::new(&settings.database_url).await?);

    // Initialize services
    let user_repo = Arc::new(SqlUserRepository::new(db_pool.clone()));
    let user_service = Arc::new(UserService::new(user_repo));

    let state = Arc::new(AppState {
        db_pool,
        settings: settings.clone(),
        user_service,
    });

    // 3. Server Setup
    let app = create_app(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], settings.server_port));
    let listener = tokio::net::TcpListener::bind(addr).await?;

    tracing::info!("Listening on {}", addr);

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            tracing::info!("Ctrl+C received, shutting down gracefully...");
        },
        _ = terminate => {
            tracing::info!("SIGTERM received, shutting down gracefully...");
        },
    }
}
