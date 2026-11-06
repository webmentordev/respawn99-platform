use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

use crate::{
    error::{AppError, Result},
    state::AppState,
};

macro_rules! select {
    ($tail:literal) => {
        concat!(
            "SELECT a.id, a.game_id, g.slug AS game_slug, g.name AS game_name,
        a.platform_id, p.icon AS platform_icon, p.name AS platform_name,
        a.title, a.description, a.hours, a.level, a.price_cents,
        (a.price_cents / 100.0)::float8 AS price, a.stock, a.is_published,
        COALESCE(
            (SELECT array_agg(i.label ORDER BY i.sort_order)::text[]
             FROM account_items i WHERE i.account_id = a.id),
            ARRAY[]::text[]
        ) AS items,
        a.created_at, a.updated_at
    FROM accounts a
    JOIN games g ON g.id = a.game_id
    JOIN platforms p ON p.id = a.platform_id",
            " ",
            $tail
        )
    };
}

#[derive(Serialize, FromRow)]
pub struct AccountView {
    pub id: Uuid,
    pub game_id: Uuid,
    #[serde(rename = "game")]
    pub game_slug: String,
    pub game_name: String,
    pub platform_id: Uuid,
    #[serde(rename = "platform")]
    pub platform_icon: String,
    #[serde(rename = "platformName")]
    pub platform_name: String,
    pub title: String,
    pub description: Option<String>,
    pub hours: i32,
    pub level: Option<i32>,
    pub price_cents: i32,
    pub price: f64,
    pub stock: i32,
    pub is_published: bool,
    pub items: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct ListQuery {
    pub game: Option<String>,
}

#[derive(Deserialize)]
pub struct AccountInput {
    pub game_id: Uuid,
    pub platform_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    #[serde(default)]
    pub hours: i32,
    pub level: Option<i32>,
    pub price_cents: i32,
    #[serde(default = "one")]
    pub stock: i32,
    #[serde(default = "yes")]
    pub is_published: bool,
    #[serde(default)]
    pub items: Vec<String>,
}

fn one() -> i32 {
    1
}

fn yes() -> bool {
    true
}

async fn fetch(db: &PgPool, id: Uuid) -> Result<AccountView> {
    let row = sqlx::query_as::<_, AccountView>(select!("WHERE a.id = $1"))
        .bind(id)
        .fetch_one(db)
        .await?;
    Ok(row)
}

fn clean_items(items: &[String]) -> Vec<String> {
    items
        .iter()
        .map(|v| v.trim().to_string())
        .filter(|v| !v.is_empty())
        .collect()
}

fn validate(i: &AccountInput) -> Result<()> {
    if i.title.trim().is_empty() {
        return Err(AppError::BadRequest("title is required".to_string()));
    }
    if i.price_cents < 0 || i.stock < 0 || i.hours < 0 {
        return Err(AppError::BadRequest(
            "price, stock and hours must not be negative".to_string(),
        ));
    }
    Ok(())
}

pub async fn public_list(
    State(s): State<AppState>,
    Query(q): Query<ListQuery>,
) -> Result<Json<Vec<AccountView>>> {
    let rows = sqlx::query_as::<_, AccountView>(select!(
        "WHERE a.is_published AND g.is_active AND ($1::text IS NULL OR g.slug = $1) ORDER BY a.created_at DESC"
    ))
    .bind(q.game)
    .fetch_all(&s.db)
    .await?;
    Ok(Json(rows))
}

pub async fn public_get(
    State(s): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<AccountView>> {
    let row = fetch(&s.db, id).await?;
    if !row.is_published {
        return Err(AppError::NotFound);
    }
    Ok(Json(row))
}

pub async fn admin_list(State(s): State<AppState>) -> Result<Json<Vec<AccountView>>> {
    let rows = sqlx::query_as::<_, AccountView>(select!("ORDER BY a.created_at DESC"))
        .fetch_all(&s.db)
        .await?;
    Ok(Json(rows))
}

pub async fn admin_get(
    State(s): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<AccountView>> {
    Ok(Json(fetch(&s.db, id).await?))
}

pub async fn create(
    State(s): State<AppState>,
    Json(i): Json<AccountInput>,
) -> Result<(StatusCode, Json<AccountView>)> {
    validate(&i)?;
    let items = clean_items(&i.items);

    let mut tx = s.db.begin().await?;

    let id: Uuid = sqlx::query_scalar(
        "INSERT INTO accounts
            (game_id, platform_id, title, description, hours, level, price_cents, stock, is_published)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
         RETURNING id",
    )
    .bind(i.game_id)
    .bind(i.platform_id)
    .bind(i.title.trim())
    .bind(i.description)
    .bind(i.hours)
    .bind(i.level)
    .bind(i.price_cents)
    .bind(i.stock)
    .bind(i.is_published)
    .fetch_one(&mut *tx)
    .await?;

    sqlx::query(
        "INSERT INTO account_items (account_id, label, sort_order)
         SELECT $1, t.label, (t.ord - 1)::int
         FROM UNNEST($2::text[]) WITH ORDINALITY AS t(label, ord)",
    )
    .bind(id)
    .bind(&items)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok((StatusCode::CREATED, Json(fetch(&s.db, id).await?)))
}

pub async fn update(
    State(s): State<AppState>,
    Path(id): Path<Uuid>,
    Json(i): Json<AccountInput>,
) -> Result<Json<AccountView>> {
    validate(&i)?;
    let items = clean_items(&i.items);

    let mut tx = s.db.begin().await?;

    let res = sqlx::query(
        "UPDATE accounts
         SET game_id = $2, platform_id = $3, title = $4, description = $5, hours = $6,
             level = $7, price_cents = $8, stock = $9, is_published = $10, updated_at = NOW()
         WHERE id = $1",
    )
    .bind(id)
    .bind(i.game_id)
    .bind(i.platform_id)
    .bind(i.title.trim())
    .bind(i.description)
    .bind(i.hours)
    .bind(i.level)
    .bind(i.price_cents)
    .bind(i.stock)
    .bind(i.is_published)
    .execute(&mut *tx)
    .await?;

    if res.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }

    sqlx::query("DELETE FROM account_items WHERE account_id = $1")
        .bind(id)
        .execute(&mut *tx)
        .await?;

    sqlx::query(
        "INSERT INTO account_items (account_id, label, sort_order)
         SELECT $1, t.label, (t.ord - 1)::int
         FROM UNNEST($2::text[]) WITH ORDINALITY AS t(label, ord)",
    )
    .bind(id)
    .bind(&items)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(Json(fetch(&s.db, id).await?))
}

pub async fn remove(State(s): State<AppState>, Path(id): Path<Uuid>) -> Result<StatusCode> {
    let res = sqlx::query("DELETE FROM accounts WHERE id = $1")
        .bind(id)
        .execute(&s.db)
        .await?;
    if res.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }
    Ok(StatusCode::NO_CONTENT)
}
