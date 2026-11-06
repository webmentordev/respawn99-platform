mod apis;
mod auth;
mod config;
mod error;
mod state;

use std::sync::Arc;

use sqlx::{PgPool, postgres::PgPoolOptions};
use tower_http::{
    cors::CorsLayer,
    services::{ServeDir, ServeFile},
};

use crate::{config::Config, state::AppState};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    let cfg = Config::from_env()?;

    let db = PgPoolOptions::new()
        .max_connections(10)
        .connect(&cfg.database_url)
        .await?;

    sqlx::migrate!("./migrations").run(&db).await?;
    ensure_admin(&db, &cfg).await?;

    let state = AppState {
        db,
        jwt_secret: Arc::from(cfg.jwt_secret.as_str()),
    };

    let mut app = apis::router(state).layer(CorsLayer::permissive());

    if let Some(dir) = &cfg.static_dir {
        app = app.fallback_service(
            ServeDir::new(dir).fallback(ServeFile::new(format!("{dir}/index.html"))),
        );
    }

    let listener = tokio::net::TcpListener::bind(&cfg.bind_addr).await?;
    println!("listening on {}", cfg.bind_addr);
    axum::serve(listener, app).await?;
    Ok(())
}

async fn ensure_admin(db: &PgPool, cfg: &Config) -> anyhow::Result<()> {
    let (Some(email), Some(password)) = (&cfg.admin_email, &cfg.admin_password) else {
        return Ok(());
    };

    let exists: bool = sqlx::query_scalar("SELECT EXISTS (SELECT 1 FROM users WHERE is_admin)")
        .fetch_one(db)
        .await?;
    if exists {
        return Ok(());
    }

    let hash = auth::hash_password(password)?;
    sqlx::query(
        "INSERT INTO users (name, email, password_hash, is_admin) VALUES ('Admin', $1, $2, TRUE)",
    )
    .bind(email.trim().to_lowercase())
    .bind(hash)
    .execute(db)
    .await?;
    Ok(())
}
