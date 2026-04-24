mod database;
mod downloader;
mod env;
mod server;
mod util;

pub use database::*;
pub use downloader::*;
pub use env::*;
pub use server::*;
pub use util::*;

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use log::{info, warn};
use std::{
    net::{Ipv4Addr, SocketAddr},
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::{
    net::TcpListener,
    select,
    signal::unix::{SignalKind, signal},
    time::sleep,
};
use tokio_util::sync::CancellationToken;

#[derive(Clone)]
pub struct AppState {
    pub downloader: Arc<Downloader>,
    pub database: Arc<Database>,
    pub cancel_token: CancellationToken,
}

impl AppState {
    pub fn new(
        downloader: Downloader,
        database: Database,
        cancel_token: CancellationToken,
    ) -> Self {
        AppState {
            downloader: Arc::new(downloader),
            database: Arc::new(database),
            cancel_token,
        }
    }
}

async fn task_signal(app_state: AppState) {
    let mut sigterm = signal(SignalKind::interrupt()).unwrap();
    let mut sigint = signal(SignalKind::terminate()).unwrap();
    select! {
        _ = sigterm.recv() => { app_state.cancel_token.cancel() },
        _ = sigint.recv() => { app_state.cancel_token.cancel() },
        _ = app_state.cancel_token.cancelled() => {}
    }
}

async fn refresh_database(app_state: &AppState) -> Result<()> {
    const REFRESH_DURATION: chrono::Duration = chrono::Duration::days(1);
    let now = Utc::now();
    let last_updated = app_state
        .database
        .get_metadata()
        .await?
        .map(|x| x.last_updated)
        .unwrap_or(DateTime::UNIX_EPOCH);
    if now - last_updated > REFRESH_DURATION {
        let mut start = Instant::now();
        info!("Fetching cards from scryfall...");
        let cards = app_state.downloader.fetch().await?;
        info!("Fetched {} cards in {:?}", cards.len(), start.elapsed());
        start = Instant::now();
        info!("Populating database...");
        app_state.database.insert_cards(&cards).await?;
        info!("Populated database in {:?}", start.elapsed());
        app_state
            .database
            .set_metadata(Metadata { last_updated: now })
            .await?;
    }
    Ok(())
}

async fn task_refresh_database(app_state: AppState) {
    const SLEEP_DURATION: Duration = Duration::from_secs(60);
    loop {
        if let Err(err) = refresh_database(&app_state).await {
            warn!("failed to refresh card database: {err}");
        }

        select! {
            _ = app_state.cancel_token.cancelled() => { break; }
            _ = sleep(SLEEP_DURATION) => {}
        }
    }
}

pub fn init_logger() {
    env_logger::builder()
        .format_timestamp(None)
        .format_target(false)
        .filter_level(log::LevelFilter::Info)
        .parse_default_env()
        .init();
}

pub async fn run() -> Result<()> {
    let env = Env::load()?;

    let downloader = Downloader::new()?;
    let database = Database::open(&env.database_file).await?;
    let cancel_token = CancellationToken::new();
    let app_state = AppState::new(downloader, database, cancel_token);

    app_state.database.init().await?;

    tokio::spawn(task_signal(app_state.clone()));
    tokio::spawn(task_refresh_database(app_state.clone()));

    let router = build_router(app_state.clone(), &env.public_dir);
    let listener =
        TcpListener::bind(&SocketAddr::new(Ipv4Addr::new(0, 0, 0, 0).into(), env.port)).await?;
    info!("Starting server on port {}", env.port);
    let result = axum::serve(listener, router)
        .with_graceful_shutdown(async move { app_state.cancel_token.cancelled().await })
        .await;

    match result {
        Ok(()) => {
            info!("Server shutdown gracefully");
            Ok(())
        }
        Err(e) => Err(e).context("server failed to shutdown"),
    }
}
