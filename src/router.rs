use axum::{
    http::{Method, StatusCode},
    response::IntoResponse,
    routing::get,
    Router,
};

use crate::{app::AppState, util::errors::not_found};

pub fn build_axum_router(state: AppState) -> Router<()> {
    let mut router = Router::new().route("/api", get(|| async { "api interface" }));

    router
        .fallback(|method| async move {
            match method {
                Method::HEAD => StatusCode::NOT_FOUND.into_response(),
                _ => not_found().into_response(),
            }
        })
        .with_state(state)
}
