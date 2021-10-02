use rocket::http::{ContentType, Status};
use rocket::request::Request;
use rocket::response::{self, Responder, Response};
use rocket::serde::json::serde_json::json;
use rocket::serde::json::Value as JsonValue;
use std::io::Cursor;

#[derive(Debug)]
pub struct ApiResponse {
    status: Status,
    message: JsonValue,
}

impl ApiResponse {
    pub fn ok(message: JsonValue) -> Self {
        ApiResponse {
            status: Status::Ok,
            message: message,
        }
    }

    pub fn empty_ok() -> Self {
        ApiResponse {
            status: Status::Ok,
            message: json!({"code": 0}),
        }
    }

    // pub fn err(message: JsonValue) -> Self {
    //     ApiResponse {
    //         status: Status::InternalServerError,
    //         message: message,
    //     }
    // }

    pub fn internal_err() -> Self {
        ApiResponse {
            status: Status::InternalServerError,
            message: json!({"code": -1}),
        }
    }
}

impl<'r> Responder<'r, 'static> for ApiResponse {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        let message = self.message.to_string();
        Response::build()
            .status(self.status)
            .header(ContentType::JSON)
            .sized_body(message.len(), Cursor::new(message))
            .ok()
    }
}
