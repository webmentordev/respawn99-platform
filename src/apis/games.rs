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
pub struct Game {
    pub id: Uuid,
    pub slug: String,
    pub name: String,
    pub tag: String,
    pub tint: String,
    pub ring: String,
    pub dot: String,
    pub sort_order: i32,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, FromRow)]
pub struct GameCard {
    pub id: Uuid,
    pub slug: String,
    pub name: String,
    pub tag: String,
    pub tint: String,
    pub ring: String,
    pub dot: String,
    pub count: i64,
    #[serde(rename = "from")]
    pub from_price: Option<i32>,
}

#[derive(Deserialize)]
pub struct GameInput {
    pub slug: String,
    pub name: String,
    pub tag: String,
    pub tint: String,
    pub ring: String,
    pub dot: String,
    pub sort_order: Option<i32>,
    pub is_active: Option<bool>,
}

pub async fn public_list(State(s): State<AppState>) -> Result<Json<Vec<GameCard>>> {
    let rows = sqlx::query_as::<_, GameCard>(
        "SELECT g.id, g.slug, g.name, g.tag, g.tint, g.ring, g.dot,
                COUNT(a.id) FILTER (WHERE a.is_published AND a.stock > 0) AS count,
                (MIN(a.price_cents) FILTER (WHERE a.is_published AND a.stock > 0) / 100)::int4 AS from_price
         FROM games g
         LEFT JOIN accounts a ON a.game_id = g.id
         WHERE g.is_active
         GROUP BY g.id
         ORDER BY g.sort_order, g.name",
    )
    .fetch_all(&s.db)
    .await?;
    Ok(Json(rows))
}

pub async fn admin_list(State(s): State<AppState>) -> Result<Json<Vec<Game>>> {
    let rows = sqlx::query_as::<_, Game>("SELECT * FROM games ORDER BY sort_order, name")
        .fetch_all(&s.db)
        .await?;
    Ok(Json(rows))
}

pub async fn create(
    State(s): State<AppState>,
    Json(i): Json<GameInput>,
) -> Result<(StatusCode, Json<Game>)> {
    if i.slug.trim().is_empty() || i.name.trim().is_empty() {
        return Err(AppError::BadRequest(
            "slug and name are required".to_string(),
        ));
    }
    let row = sqlx::query_as::<_, Game>(
        "INSERT INTO games (slug, name, tag, tint, ring, dot, sort_order, is_active)
         VALUES ($1, $2, $3, $4, $5, $6, COALESCE($7, 0), COALESCE($8, TRUE))
         RETURNING *",
    )
    .bind(i.slug.trim())
    .bind(i.name.trim())
    .bind(i.tag)
    .bind(i.tint)
    .bind(i.ring)
    .bind(i.dot)
    .bind(i.sort_order)
    .bind(i.is_active)
    .fetch_one(&s.db)
    .await?;
    Ok((StatusCode::CREATED, Json(row)))
}

pub async fn update(
    State(s): State<AppState>,
    Path(id): Path<Uuid>,
    Json(i): Json<GameInput>,
) -> Result<Json<Game>> {
    let row = sqlx::query_as::<_, Game>(
        "UPDATE games
         SET slug = $2, name = $3, tag = $4, tint = $5, ring = $6, dot = $7,
             sort_order = COALESCE($8, sort_order),
             is_active = COALESCE($9, is_active),
             updated_at = NOW()
         WHERE id = $1
         RETURNING *",
    )
    .bind(id)
    .bind(i.slug.trim())
    .bind(i.name.trim())
    .bind(i.tag)
    .bind(i.tint)
    .bind(i.ring)
    .bind(i.dot)
    .bind(i.sort_order)
    .bind(i.is_active)
    .fetch_one(&s.db)
    .await?;
    Ok(Json(row))
}

pub async fn remove(State(s): State<AppState>, Path(id): Path<Uuid>) -> Result<StatusCode> {
    let res = sqlx::query("DELETE FROM games WHERE id = $1")
        .bind(id)
        .execute(&s.db)
        .await?;
    if res.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }
    Ok(StatusCode::NO_CONTENT)
}
