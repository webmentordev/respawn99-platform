use std::env;

pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub bind_addr: String,
    pub static_dir: Option<String>,
    pub admin_email: Option<String>,
    pub admin_password: Option<String>,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        Ok(Self {
            database_url: env::var("DATABASE_URL")?,
            jwt_secret: env::var("JWT_SECRET")?,
            bind_addr: env::var("BIND_ADDR").unwrap_or_else(|_| "0.0.0.0:3000".to_string()),
            static_dir: env::var("STATIC_DIR").ok().filter(|v| !v.is_empty()),
            admin_email: env::var("ADMIN_EMAIL").ok().filter(|v| !v.is_empty()),
            admin_password: env::var("ADMIN_PASSWORD").ok().filter(|v| !v.is_empty()),
        })
    }
}
