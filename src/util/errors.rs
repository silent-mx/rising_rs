use core::fmt;
use std::any::{Any, TypeId};

use axum::{http::StatusCode, response::IntoResponse};
use json::custom;

mod json;

pub trait AppError: Send + fmt::Display + fmt::Debug + 'static {
    fn response(&self) -> axum::response::Response;

    fn get_type_id(&self) -> TypeId {
        TypeId::of::<Self>()
    }
}

impl dyn AppError {
    pub fn is<T: Any>(&self) -> bool {
        self.get_type_id() == TypeId::of::<T>()
    }
}

pub type BoxedAppError = Box<dyn AppError>;
impl AppError for BoxedAppError {
    fn response(&self) -> axum::response::Response {
        (**self).response()
    }

    fn get_type_id(&self) -> TypeId {
        (**self).get_type_id()
    }
}

impl IntoResponse for BoxedAppError {
    fn into_response(self) -> axum::response::Response {
        self.response()
    }
}

pub fn not_found() -> BoxedAppError {
    custom(StatusCode::NOT_FOUND, "Not Found")
}
