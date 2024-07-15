use axum::routing::get;

use crate::app_state::AppState;

mod sys_user;

pub fn router(state: AppState) -> axum::Router {
    axum::Router::new()
        .route("/sys_users", get(sys_user::sys_users))
        .with_state(state)
}
