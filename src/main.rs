use std::sync::Arc;

use axum::response::Redirect;
use clap::{Arg, Command};
use once_cell::sync::Lazy;
use surrealdb::{engine::remote::ws::Client, Surreal};
use tokio::{net::TcpListener, signal, sync::Mutex};
use tower_http::trace::TraceLayer;

use anyhow::Result;

use config::EnvConfig;
use log::{error, info};

mod askama_templates;
mod config;
mod db;
mod models;
mod routes;
mod services;

static IS_SHUTDOWN: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));

const DEFAULT_CONFIG_FILE: &str = ".env";

const SERVICE_NAME: &str = "Parami Centre Exam System";

#[derive(Clone)]
pub struct AppState {
    db: Arc<Mutex<Surreal<Client>>>,
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    // Initialize tracing subscriber for logging

    pretty_env_logger::formatted_builder()
        .filter_level(log::LevelFilter::Debug)
        .format_timestamp_millis()
        .filter("hyper".into(), log::LevelFilter::Warn)
        .init();

    // CLI arguments
    let matches = Command::new(SERVICE_NAME)
        .version("0.1.0")
        .about(SERVICE_NAME)
        .arg(
            Arg::new("env")
                .short('e')
                .long("env")
                .value_name("FILE")
                .help("Sets the .env file to use")
                .default_value(DEFAULT_CONFIG_FILE)
                .required(false),
        )
        .get_matches();

    let env_file_result = matches.get_one::<String>("env");
    let env_file = match env_file_result {
        Some(file) => file,
        None => {
            error!("Failed to get the .env file");
            return Err(());
        }
    };

    info!("Loading the environment configuration from {}", env_file);

    let env_config: EnvConfig;

    match EnvConfig::new(env_file) {
        Ok(config) => env_config = config,
        Err(e) => {
            error!("{}", e);
            return Err(());
        }
    }

    // Connect to the database
    let db_result = db::connection::establish_connection(&env_config).await;

    if let Err(e) = db_result {
        error!("Failed to connect to the database: {}", e);
        return Err(());
    }

    let db = db_result.unwrap();
    let app_state = AppState {
        db: Arc::new(Mutex::new(db)),
        // html_templates: Arc::new(TEMPLATES.clone()),
    };

    // Create the Axum router
    let app = routes::routes()
        .route("/", axum::routing::get(|| async { Redirect::to("/exam") }))
        .with_state(app_state)
        .layer(TraceLayer::new_for_http());

    // Define the address and port to listen on
    let listener = TcpListener::bind((
        env_config.server_address.as_str(),
        env_config.server_port.parse::<u16>().unwrap(),
    ))
    .await
    .map_err(|e| {
        error!("Failed to bind to the address: {}", e);
    })?;

    info!("Listening on http://{}", listener.local_addr().unwrap(),);

    // Spawn the Axum server as a separate task
    let axum_server = tokio::spawn(async move {
        info!("Starting Axum Web server...");
        if let Err(e) = axum::serve(listener, app.into_make_service())
            .with_graceful_shutdown(shutdown_signal())
            .await
        {
            error!("Axum Web server error: {}", e);
        }
    });

    if let Err(e) = tokio::try_join!(axum_server) {
        error!(
            "Failed to start the Axum server and the Telegram bot: {}",
            e
        );
        return Err(());
    }
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
            info!("Ctrl+C received, shutting down...");
            *IS_SHUTDOWN.lock().await = true;
        },
        _ = terminate => {
            info!("Terminate signal received, shutting down...");
            *IS_SHUTDOWN.lock().await = true;
        },
    }
}
