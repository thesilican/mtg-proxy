mod database;
mod downloader;
mod server;

use anyhow::{bail, Context, Result};
use database::Database;
use downloader::Downloader;
use log::{info, warn};
use server::AppState;
use std::{
    env,
    net::{Ipv4Addr, SocketAddr},
    time::Duration,
};
use tokio::{
    net::TcpListener,
    select,
    signal::unix::{signal, SignalKind},
    time::sleep,
};
use tokio_util::sync::CancellationToken;

use crate::server::build_router;

async fn handle_ctrl_c(cancel_token: CancellationToken) {
    let mut sigterm = signal(SignalKind::interrupt()).unwrap();
    let mut sigint = signal(SignalKind::terminate()).unwrap();
    select! {
        _ = sigterm.recv() => (),
        _ = sigint.recv() => (),
    };
    cancel_token.cancel();
}

async fn update_loop(app_state: AppState, cancel_token: CancellationToken) {
    const SLEEP_DURATION: Duration = Duration::from_secs(60 * 60 * 24 * 7);
    loop {
        let result = app_state
            .downloader
            .refresh_database(app_state.database.clone())
            .await;
        if let Err(err) = result {
            warn!("failed to refresh card database: {err}");
        }

        select! {
            _ = cancel_token.cancelled() => { break; }
            _ = sleep(SLEEP_DURATION) => {}
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::builder()
        .format_timestamp(None)
        .format_target(false)
        .filter_level(log::LevelFilter::Info)
        .parse_default_env()
        .init();

    let port = env::var("PORT").unwrap_or("8080".to_string()).parse()?;
    let public_dir = env::var("PUBLIC_DIR").unwrap_or("../frontend/dist".to_string());
    let database_file = env::var("DATABASE_FILE").unwrap_or("./data/database.db".to_string());

    let downloader = Downloader::new().context("failed to create downloader")?;
    let database = Database::open(&database_file)
        .await
        .context("failed to open database")?;
    database.init().await.context("failed to init database")?;
    let app_state = AppState::new(downloader, database);

    let cancel_token = CancellationToken::new();
    tokio::spawn(handle_ctrl_c(cancel_token.clone()));
    tokio::spawn(update_loop(app_state.clone(), cancel_token.clone()));

    let app = build_router(app_state.clone(), &public_dir);

    info!("Starting server on port {}", port);

    let listener =
        TcpListener::bind(&SocketAddr::new(Ipv4Addr::new(0, 0, 0, 0).into(), port)).await?;

    let cancel_token = cancel_token.clone();
    let result = axum::serve(listener, app)
        .with_graceful_shutdown(async move { cancel_token.cancelled().await })
        .await;

    match result {
        Ok(()) => {
            info!("Server shutdown gracefully");
            Ok(())
        }
        Err(e) => bail!("server did not shut down gracefully: {e}"),
    }
}
