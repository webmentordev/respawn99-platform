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
pub struct OtherGame {
    pub id: Uuid,
    pub name: String,
    pub sort_order: i32,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct OtherGameInput {
    pub name: String,
    pub sort_order: Option<i32>,
    pub is_active: Option<bool>,
}

pub async fn public_list(State(s): State<AppState>) -> Result<Json<Vec<OtherGame>>> {
    let rows = sqlx::query_as::<_, OtherGame>(
        "SELECT * FROM other_games WHERE is_active ORDER BY sort_order, name",
    )
    .fetch_all(&s.db)
    .await?;
    Ok(Json(rows))
}

pub async fn admin_list(State(s): State<AppState>) -> Result<Json<Vec<OtherGame>>> {
    let rows =
        sqlx::query_as::<_, OtherGame>("SELECT * FROM other_games ORDER BY sort_order, name")
            .fetch_all(&s.db)
            .await?;
    Ok(Json(rows))
}

pub async fn create(
    State(s): State<AppState>,
    Json(i): Json<OtherGameInput>,
) -> Result<(StatusCode, Json<OtherGame>)> {
    if i.name.trim().is_empty() {
        return Err(AppError::BadRequest("name is required".to_string()));
    }
    let row = sqlx::query_as::<_, OtherGame>(
        "INSERT INTO other_games (name, sort_order, is_active)
         VALUES ($1, COALESCE($2, 0), COALESCE($3, TRUE))
         RETURNING *",
    )
    .bind(i.name.trim())
    .bind(i.sort_order)
    .bind(i.is_active)
    .fetch_one(&s.db)
    .await?;
    Ok((StatusCode::CREATED, Json(row)))
}

pub async fn update(
    State(s): State<AppState>,
    Path(id): Path<Uuid>,
    Json(i): Json<OtherGameInput>,
) -> Result<Json<OtherGame>> {
    let row = sqlx::query_as::<_, OtherGame>(
        "UPDATE other_games
         SET name = $2,
             sort_order = COALESCE($3, sort_order),
             is_active = COALESCE($4, is_active)
         WHERE id = $1
         RETURNING *",
    )
    .bind(id)
    .bind(i.name.trim())
    .bind(i.sort_order)
    .bind(i.is_active)
    .fetch_one(&s.db)
    .await?;
    Ok(Json(row))
}

pub async fn remove(State(s): State<AppState>, Path(id): Path<Uuid>) -> Result<StatusCode> {
    let res = sqlx::query("DELETE FROM other_games WHERE id = $1")
        .bind(id)
        .execute(&s.db)
        .await?;
    if res.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }
    Ok(StatusCode::NO_CONTENT)
}
