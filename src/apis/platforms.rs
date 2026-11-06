use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use crate::{
    error::{AppError, Result},
    state::AppState,
};

#[derive(Serialize, FromRow)]
pub struct Platform {
    pub id: Uuid,
    pub slug: String,
    pub name: String,
    pub icon: String,
    pub sort_order: i32,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct PlatformInput {
    pub slug: String,
    pub name: String,
    pub icon: String,
    pub sort_order: Option<i32>,
    pub is_active: Option<bool>,
}

pub async fn public_list(State(s): State<AppState>) -> Result<Json<Vec<Platform>>> {
    let rows = sqlx::query_as::<_, Platform>(
        "SELECT * FROM platforms WHERE is_active ORDER BY sort_order, name",
    )
    .fetch_all(&s.db)
    .await?;
    Ok(Json(rows))
}

pub async fn admin_list(State(s): State<AppState>) -> Result<Json<Vec<Platform>>> {
    let rows = sqlx::query_as::<_, Platform>("SELECT * FROM platforms ORDER BY sort_order, name")
        .fetch_all(&s.db)
        .await?;
    Ok(Json(rows))
}

pub async fn create(
    State(s): State<AppState>,
    Json(i): Json<PlatformInput>,
) -> Result<(StatusCode, Json<Platform>)> {
    if i.slug.trim().is_empty() || i.name.trim().is_empty() {
        return Err(AppError::BadRequest(
            "slug and name are required".to_string(),
        ));
    }
    let row = sqlx::query_as::<_, Platform>(
        "INSERT INTO platforms (slug, name, icon, sort_order, is_active)
         VALUES ($1, $2, $3, COALESCE($4, 0), COALESCE($5, TRUE))
         RETURNING *",
    )
    .bind(i.slug.trim())
    .bind(i.name.trim())
    .bind(i.icon)
    .bind(i.sort_order)
    .bind(i.is_active)
    .fetch_one(&s.db)
    .await?;
    Ok((StatusCode::CREATED, Json(row)))
}

pub async fn update(
    State(s): State<AppState>,
    Path(id): Path<Uuid>,
    Json(i): Json<PlatformInput>,
) -> Result<Json<Platform>> {
    let row = sqlx::query_as::<_, Platform>(
        "UPDATE platforms
         SET slug = $2, name = $3, icon = $4,
             sort_order = COALESCE($5, sort_order),
             is_active = COALESCE($6, is_active),
             updated_at = NOW()
         WHERE id = $1
         RETURNING *",
    )
    .bind(id)
    .bind(i.slug.trim())
    .bind(i.name.trim())
    .bind(i.icon)
    .bind(i.sort_order)
    .bind(i.is_active)
    .fetch_one(&s.db)
    .await?;
    Ok(Json(row))
}

pub async fn remove(State(s): State<AppState>, Path(id): Path<Uuid>) -> Result<StatusCode> {
    let res = sqlx::query("DELETE FROM platforms WHERE id = $1")
        .bind(id)
        .execute(&s.db)
        .await?;
    if res.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }
    Ok(StatusCode::NO_CONTENT)
}
