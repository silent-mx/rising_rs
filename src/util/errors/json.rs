use std::{borrow::Cow, fmt};

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

use super::{AppError, BoxedAppError};

#[derive(Debug, Clone)]
pub struct CustomApiError {
    status: StatusCode,
    detail: Cow<'static, str>,
}

impl fmt::Display for CustomApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.detail.fmt(f)
    }
}

impl AppError for CustomApiError {
    fn response(&self) -> Response {
        json_error(&self.detail, self.status)
    }
}

/// Generates a response with the provided status and description as JSON
fn json_error(detail: &str, status: StatusCode) -> Response {
    let json = json!({ "errors": [{ "detail": detail }] });
    (status, Json(json)).into_response()
}

pub fn custom(status: StatusCode, detail: impl Into<Cow<'static, str>>) -> BoxedAppError {
    Box::new(CustomApiError {
        status,
        detail: detail.into(),
    })
}
