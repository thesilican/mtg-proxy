use anyhow::{Context, Result};
use axum::{
    middleware::from_fn,
    routing::{get, post},
    Router,
};
use log::{info, LevelFilter};
use mtg_proxy::{get_ping, log_middleware, post_print};
use std::{future::IntoFuture, path::PathBuf};
use tokio::{net::TcpListener, select, signal::ctrl_c};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    env_logger::builder()
        .filter_level(LevelFilter::Info)
        .format_timestamp(None)
        .format_target(false)
        .parse_default_env()
        .init();

    let public_dir = PathBuf::from(option_env!("PUBLIC_DIR").unwrap_or("frontend/dist"))
        .canonicalize()
        .context("Could not load PUBLIC_DIR")?;
    info!("Serving files from {}", public_dir.to_string_lossy());
    let file_serve = ServeDir::new(public_dir);

    let app = Router::new()
        .route("/api/ping", get(get_ping))
        .route("/api/print", post(post_print))
        .fallback_service(file_serve)
        .layer(from_fn(log_middleware));

    let port = std::env::var("PORT")
        .map(|arg| arg.parse::<u16>().ok())
        .ok()
        .flatten()
        .unwrap_or(8080);
    info!("Starting server on port {port}");
    let listener = TcpListener::bind(("0.0.0.0", port)).await?;
    let server = axum::serve(listener, app).into_future();

    select! {
        res = server => {
            res.context("error running server")
        }
        res = ctrl_c() => {
            match res {
                Ok(()) => {
                    info!("Stopping server");
                    Ok(())
                },
                Err(err) => Err(err).context("error joining timer")
            }
        }
    }
}
