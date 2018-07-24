use actix_web::{HttpRequest};

use common::state::AppState;
use common::filter::Unauthorized;
use models::response::{Message, MessageResult};

pub fn handle_error(req: &HttpRequest<AppState>) -> MessageResult<String> {

    match req.extensions().get::<Unauthorized>() {
        Some(_) => {
            Message::error("用户未登录")
        },
        None => {
            Message::error("资源不存在")
        }
    }
}

pub fn not_found(_req: &HttpRequest) -> MessageResult<String> {

    Message::error("资源不存在")
}