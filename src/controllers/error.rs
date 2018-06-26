use actix_web::{HttpRequest};

use common::state::AppState;
use models::response::{Message, MessageResult};

pub fn not_found(_req: HttpRequest<AppState>) -> MessageResult<String> {

    Message::error("not found")
}

pub fn global_not_found(_req: HttpRequest) -> MessageResult<String> {

    Message::error("not found")
}