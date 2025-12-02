use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub port: u16,
    pub database_url: String,
    pub jwt: JwtConfig,
}

#[derive(Debug, Clone)]
pub struct JwtConfig {
    pub access_secret: String,
    pub refresh_secret: String,
    pub access_expiry_seconds: i64,
    pub refresh_expiry_seconds: i64,
}

impl Config {
    pub fn from_env() -> Result<Self, String> {
        let port = env::var("PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse()
            .map_err(|_| "Invalid PORT value")?;

        let database_url = env::var("DATABASE_URL")
            .map_err(|_| "DATABASE_URL must be set")?;

        let jwt = JwtConfig {
            access_secret: env::var("JWT_ACCESS_SECRET")
                .map_err(|_| "JWT_ACCESS_SECRET must be set")?,
            refresh_secret: env::var("JWT_REFRESH_SECRET")
                .map_err(|_| "JWT_REFRESH_SECRET must be set")?,
            access_expiry_seconds: env::var("JWT_ACCESS_EXPIRY_SECONDS")
                .unwrap_or_else(|_| "900".to_string())
                .parse()
                .map_err(|_| "Invalid JWT_ACCESS_EXPIRY_SECONDS")?,
            refresh_expiry_seconds: env::var("JWT_REFRESH_EXPIRY_SECONDS")
                .unwrap_or_else(|_| "604800".to_string())
                .parse()
                .map_err(|_| "Invalid JWT_REFRESH_EXPIRY_SECONDS")?,
        };

        Ok(Config {
            port,
            database_url,
            jwt,
        })
    }
}
