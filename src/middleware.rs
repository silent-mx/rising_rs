use std::time::Duration;

use axum::Router;
use tower_http::{
    compression::CompressionLayer,
    timeout::{RequestBodyTimeoutLayer, TimeoutLayer},
    CompressionLevel,
};

use crate::app::AppState;

pub fn apply_axum_middleware(state: AppState, router: Router<()>) -> Router {
    let config = &state.config;
    let env = config.env();

    router
        .layer(TimeoutLayer::new(Duration::from_secs(30)))
        .layer(RequestBodyTimeoutLayer::new(Duration::from_secs(30)))
        .layer(CompressionLayer::new().quality(CompressionLevel::Fastest))
}
