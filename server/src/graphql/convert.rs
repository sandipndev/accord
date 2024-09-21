use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::app::ApplicationError;

impl IntoResponse for ApplicationError {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self).into_response()
    }
}
