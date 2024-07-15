use crate::app_state::AppState;

mod identity;

/// 创建axum router
pub fn build_axum_router(config: crate::config::Server) -> axum::Router {
    let state = crate::app_state::AppState::new(config);

    axum::Router::new().nest("/api", mod_routers(state))
}

fn mod_routers(state: AppState) -> axum::Router {
    axum::Router::new().nest("/identity", identity::router(state))
}
