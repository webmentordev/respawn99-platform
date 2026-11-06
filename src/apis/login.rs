use axum::{Json, extract::State};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use crate::{
    auth::{create_token, verify_password},
    error::{AppError, Result},
    state::AppState,
};

#[derive(Deserialize)]
pub struct LoginInput {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginOutput {
    pub token: String,
    pub name: String,
    pub email: String,
    pub is_admin: bool,
}

#[derive(FromRow)]
struct UserRow {
    id: Uuid,
    name: String,
    email: String,
    password_hash: String,
    is_admin: bool,
}

pub async fn login(
    State(s): State<AppState>,
    Json(input): Json<LoginInput>,
) -> Result<Json<LoginOutput>> {
    let user = sqlx::query_as::<_, UserRow>(
        "SELECT id, name, email, password_hash, is_admin FROM users WHERE email = $1 AND is_active",
    )
    .bind(input.email.trim().to_lowercase())
    .fetch_optional(&s.db)
    .await?
    .ok_or(AppError::Unauthorized)?;

    if !verify_password(&input.password, &user.password_hash) {
        return Err(AppError::Unauthorized);
    }

    sqlx::query("UPDATE users SET last_login_at = NOW() WHERE id = $1")
        .bind(user.id)
        .execute(&s.db)
        .await?;

    let token = create_token(&s.jwt_secret, user.id, user.is_admin)?;

    Ok(Json(LoginOutput {
        token,
        name: user.name,
        email: user.email,
        is_admin: user.is_admin,
    }))
}
