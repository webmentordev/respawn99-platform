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
pub struct Faq {
    pub id: Uuid,
    pub question: String,
    pub answer: String,
    pub sort_order: i32,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct FaqInput {
    pub question: String,
    pub answer: String,
    pub sort_order: Option<i32>,
    pub is_active: Option<bool>,
}

pub async fn public_list(State(s): State<AppState>) -> Result<Json<Vec<Faq>>> {
    let rows = sqlx::query_as::<_, Faq>(
        "SELECT * FROM faqs WHERE is_active ORDER BY sort_order, created_at",
    )
    .fetch_all(&s.db)
    .await?;
    Ok(Json(rows))
}

pub async fn admin_list(State(s): State<AppState>) -> Result<Json<Vec<Faq>>> {
    let rows = sqlx::query_as::<_, Faq>("SELECT * FROM faqs ORDER BY sort_order, created_at")
        .fetch_all(&s.db)
        .await?;
    Ok(Json(rows))
}

pub async fn create(
    State(s): State<AppState>,
    Json(i): Json<FaqInput>,
) -> Result<(StatusCode, Json<Faq>)> {
    if i.question.trim().is_empty() || i.answer.trim().is_empty() {
        return Err(AppError::BadRequest(
            "question and answer are required".to_string(),
        ));
    }
    let row = sqlx::query_as::<_, Faq>(
        "INSERT INTO faqs (question, answer, sort_order, is_active)
         VALUES ($1, $2, COALESCE($3, 0), COALESCE($4, TRUE))
         RETURNING *",
    )
    .bind(i.question.trim())
    .bind(i.answer.trim())
    .bind(i.sort_order)
    .bind(i.is_active)
    .fetch_one(&s.db)
    .await?;
    Ok((StatusCode::CREATED, Json(row)))
}

pub async fn update(
    State(s): State<AppState>,
    Path(id): Path<Uuid>,
    Json(i): Json<FaqInput>,
) -> Result<Json<Faq>> {
    let row = sqlx::query_as::<_, Faq>(
        "UPDATE faqs
         SET question = $2, answer = $3,
             sort_order = COALESCE($4, sort_order),
             is_active = COALESCE($5, is_active),
             updated_at = NOW()
         WHERE id = $1
         RETURNING *",
    )
    .bind(id)
    .bind(i.question.trim())
    .bind(i.answer.trim())
    .bind(i.sort_order)
    .bind(i.is_active)
    .fetch_one(&s.db)
    .await?;
    Ok(Json(row))
}

pub async fn remove(State(s): State<AppState>, Path(id): Path<Uuid>) -> Result<StatusCode> {
    let res = sqlx::query("DELETE FROM faqs WHERE id = $1")
        .bind(id)
        .execute(&s.db)
        .await?;
    if res.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }
    Ok(StatusCode::NO_CONTENT)
}
