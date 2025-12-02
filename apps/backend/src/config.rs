use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub port: u16,
    pub database_url: String,
}

impl Config {
    pub fn from_env() -> Result<Self, String> {
        let port = env::var("PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse()
            .map_err(|_| "Invalid PORT value")?;

        let database_url = env::var("DATABASE_URL").map_err(|_| "DATABASE_URL must be set")?;

        Ok(Config { port, database_url })
    }
}
