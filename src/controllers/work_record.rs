use actix_web::{HttpRequest, Form};
use actix_web::middleware::session::RequestSession;

use common::state::AppState;
use models::work_record::*;
use models::response::{Message, MessageResult};

pub fn create(req: HttpRequest<AppState>, create_work_record: Form<CreateWorkRecord>) -> MessageResult<String> {

    let conn = &req.state().conn;
    let work_record = create_work_record.into_work_record();

    match work_record.create(conn, &create_work_record.events) {

        Ok(data) => {
            Message::success("success".to_owned())
        },

        Err(err) => {

            Message::error("err")
        }
    }
}

pub fn update(req: HttpRequest<AppState>, work_record: Form<WorkRecord>) -> MessageResult<String> {

    Message::success("success".to_owned())
}

pub fn get_records(req: HttpRequest<AppState>, work_record: Form<WorkRecord>) -> MessageResult<String> {

    Message::success("success".to_owned())
}

pub fn get_record(req: HttpRequest<AppState>, work_record: Form<WorkRecord>) -> MessageResult<String> {

    Message::success("success".to_owned())
}

pub fn delete(req: HttpRequest<AppState>, delete_work_record: Form<DeleteWorkRecord>) -> MessageResult<String> {

    let conn = &req.state().conn;

    match delete_work_record.delete(conn) {

        Ok(data) => {
            Message::success("success".to_owned())
        },

        Err(err) => {

            Message::error("err")
        }
    }
}
