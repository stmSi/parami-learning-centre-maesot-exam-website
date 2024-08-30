use anyhow::Result;
use log::{error, info};
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

use crate::config::EnvConfig;

const FILTERS: &'static [&str] = &["lowercase", "edgengram(1, 10)"];

pub async fn establish_connection(env_config: &EnvConfig) -> Result<Surreal<Client>, String> {
    let db_url = &env_config.surrealdb_url;
    let username = &env_config.surrealdb_username;
    let password = &env_config.surrealdb_password;
    let db_namespace = &env_config.surrealdb_namespace;
    let db_dbname = &env_config.surrealdb_database;

    // Connect to the database
    let db_result = Surreal::new::<Ws>(db_url).await;
    if let Err(e) = db_result {
        error!("Failed to connect to the database: {}", e);
        return Err(e.to_string());
    }
    let db = db_result.unwrap();

    // Sign in as the root user
    let signin_result = db.signin(Root { username, password }).await;
    if let Err(e) = signin_result {
        error!("Failed to sign in: {}", e);
        return Err(e.to_string());
    }

    // Select a specific namespace / database
    let _ = db.use_ns(db_namespace).use_db(db_dbname).await;

    info!("Connected to the Surreal database");
    Ok(db)
}
