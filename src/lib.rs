#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate tracing;

pub mod config;
pub mod db;
pub mod middleware;
pub mod models;
pub mod schema;
pub mod sentry;
pub mod util;

mod app;
mod router;

pub use crate::app::App;

use crate::app::AppState;
use crate::router::build_axum_router;
use std::sync::Arc;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Env {
    Development,
    Production,
    Test,
}

/// 配置路由、会话、日志记录和其他中间件
///
/// 从 *src/bin/server.rs* 调用
pub fn build_handler(app: Arc<App>) -> axum::Router {
    let state = AppState(app);

    let axum_router = build_axum_router(state.clone());
    middleware::apply_axum_middleware(state, axum_router)
}
