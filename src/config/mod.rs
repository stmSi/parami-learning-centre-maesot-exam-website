use anyhow::Result;

const SERVER_ADDRESS: &str = "SERVER_ADDRESS";
const SERVER_PORT: &str = "SERVER_PORT";

const SURREALDB_URL: &str = "SURREALDB_URL";
const SURREALDB_USERNAME: &str = "SURREALDB_USERNAME";
const SURREALDB_PASSWORD: &str = "SURREALDB_PASSWORD";
const SURREALDB_NAMESPACE: &str = "SURREALDB_NAMESPACE";
const SURREALDB_DBNAME: &str = "SURREALDB_DBNAME";

pub struct EnvConfig {
    pub server_address: String,
    pub server_port: String,

    pub surrealdb_url: String,
    pub surrealdb_username: String,
    pub surrealdb_password: String,
    pub surrealdb_namespace: String,
    pub surrealdb_database: String,
}

impl EnvConfig {
    pub fn new(env_file: &str) -> Result<Self, String> {
        //
        // TODO: Load different config files based on the environment e.g. .env.dev, .env.prod
        //

        // Load the .env file
        if let Err(e) = dotenvy::from_filename(env_file) {
            return Err(format!("Failed to load .env file: {}", e));
        }

        let server_address = if let Ok(address) = std::env::var(SERVER_ADDRESS) {
            address
        } else {
            return Err(format!("{} is not set in the .env file", SERVER_ADDRESS));
        };

        let server_port = if let Ok(port) = std::env::var(SERVER_PORT) {
            port
        } else {
            return Err(format!("{} is not set in the .env file", SERVER_PORT));
        };

        let surrealdb_url = if let Ok(url) = std::env::var(SURREALDB_URL) {
            url
        } else {
            // error!("SURREALDB_URL is not set in the .env file");
            return Err(format!("{} is not set in the .env file", SURREALDB_URL));
        };

        let surrealdb_username = if let Ok(username) = std::env::var(SURREALDB_USERNAME) {
            username
        } else {
            return Err(format!(
                "{} is not set in the .env file",
                SURREALDB_USERNAME
            ));
        };

        let surrealdb_password = if let Ok(password) = std::env::var(SURREALDB_PASSWORD) {
            password
        } else {
            return Err(format!(
                "{} is not set in the .env file",
                SURREALDB_PASSWORD
            ));
        };

        let surrealdb_namespace = if let Ok(namespace) = std::env::var(SURREALDB_NAMESPACE) {
            namespace
        } else {
            return Err(format!(
                "{} is not set in the .env file",
                SURREALDB_NAMESPACE
            ));
        };

        let surrealdb_database = if let Ok(database) = std::env::var(SURREALDB_DBNAME) {
            database
        } else {
            return Err(format!("{} is not set in the .env file", SURREALDB_DBNAME));
        };

        Ok(Self {
            server_address,
            server_port,
            surrealdb_url,
            surrealdb_username,
            surrealdb_password,
            surrealdb_namespace,
            surrealdb_database,
        })
    }
}
