#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate tracing;
#[macro_use]
extern crate anyhow;

// #[macro_use]
// extern crate serde_json;

use std::sync::Arc;

mod app;

pub use app::{App, AppState};
use axum::{http::StatusCode, routing::get, Json};
use diesel::prelude::*;
use models::SysUser;
pub mod config;
pub mod db;
pub mod models;
pub mod schema;
pub mod utils;

pub fn build_axum_router(app: Arc<App>) -> axum::Router {
    let state = AppState(app);
    let axum_router = axum::Router::new()
        .route("/", get(get_sys_users))
        .route("/sys_user", get(get_sys_users))
        .with_state(state);

    axum_router
}

pub async fn get_sys_users(state: AppState) -> Result<Json<Vec<SysUser>>, (StatusCode, String)> {
    let conn = state.db_write().await.unwrap();
    conn.interact(move |conn| {
        use crate::schema::sys_user::dsl::*;

        let res = sys_user
            .select(models::SysUser::as_select())
            .load(conn)
            .unwrap();
        Ok(Json(res))
    })
    .await
    .unwrap()
}
