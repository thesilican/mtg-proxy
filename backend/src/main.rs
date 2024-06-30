mod bulk_data;
mod server;

use anyhow::{bail, Result};
use bulk_data::BulkData;
use chrono::Utc;
use log::{error, info, warn};
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

async fn load_bulk_data() -> Result<BulkData> {
    let mut bulk_data = BulkData::new();
    if let Err(err) = bulk_data.load_from_file().await {
        warn!("Error loading bulk data from file: {err}");
        bulk_data.fetch().await?;
        bulk_data.save_to_file().await?;
    }
    Ok(bulk_data)
}

async fn update_loop(app_state: AppState, cancel_token: CancellationToken) {
    const SLEEP_DURATION: Duration = Duration::from_secs(60);
    loop {
        // Check if the cards are out of date
        let mut bulk_data = app_state.lock().await;
        let now = Utc::now();
        if (now - bulk_data.last_fetched).num_days() >= 7
            && (now - bulk_data.updated_at).num_days() >= 7
        {
            info!("Refreshing card database");
            if let Err(err) = bulk_data.fetch().await {
                error!("Error fetching bulk data: {err}");
            }
            if let Err(err) = bulk_data.save_to_file().await {
                error!("Error saving bulk data to file: {err}");
            }
        }
        drop(bulk_data);

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

    let port = env::var("PORT").unwrap_or(String::from("8080")).parse()?;
    let public_dir = env::var("PUBLIC_DIR").unwrap_or(String::from("../frontend/dist"));

    let bulk_data = load_bulk_data().await?;
    let app_state = AppState::new(bulk_data);

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
