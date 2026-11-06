pub mod accounts;
pub mod faqs;
pub mod games;
pub mod login;
pub mod other_games;
pub mod platforms;

use axum::{
    Router, middleware,
    routing::{get, post, put},
};

use crate::{auth::require_admin, state::AppState};

pub fn router(state: AppState) -> Router {
    let admin = Router::new()
        .route("/games", get(games::admin_list).post(games::create))
        .route("/games/{id}", put(games::update).delete(games::remove))
        .route(
            "/platforms",
            get(platforms::admin_list).post(platforms::create),
        )
        .route(
            "/platforms/{id}",
            put(platforms::update).delete(platforms::remove),
        )
        .route(
            "/other-games",
            get(other_games::admin_list).post(other_games::create),
        )
        .route(
            "/other-games/{id}",
            put(other_games::update).delete(other_games::remove),
        )
        .route("/faqs", get(faqs::admin_list).post(faqs::create))
        .route("/faqs/{id}", put(faqs::update).delete(faqs::remove))
        .route(
            "/accounts",
            get(accounts::admin_list).post(accounts::create),
        )
        .route(
            "/accounts/{id}",
            get(accounts::admin_get)
                .put(accounts::update)
                .delete(accounts::remove),
        )
        .route_layer(middleware::from_fn_with_state(state.clone(), require_admin));

    let public = Router::new()
        .route("/games", get(games::public_list))
        .route("/platforms", get(platforms::public_list))
        .route("/other-games", get(other_games::public_list))
        .route("/faqs", get(faqs::public_list))
        .route("/accounts", get(accounts::public_list))
        .route("/accounts/{id}", get(accounts::public_get))
        .route("/auth/login", post(login::login))
        .nest("/admin", admin);

    Router::new().nest("/api", public).with_state(state)
}
