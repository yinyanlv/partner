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

            Message::error("error")
        }
    }
}

pub fn update(req: HttpRequest<AppState>, update_work_record: Form<UpdateWorkRecord>) -> MessageResult<String> {

    let conn = &req.state().conn;

    match update_work_record.update(conn, &update_work_record.events) {

        Ok(data) => {

            Message::success("success".to_owned())
        },

        Err(err) => {

            Message::error("error")
        }
    }
}

pub fn get_records(req: HttpRequest<AppState>, query_month_work_record: Form<QueryMonthWorkRecord>) -> MessageResult<String> {

    let conn = &req.state().conn;
    
    match query_month_work_record.query(conn) {

        Ok(data) => {

            Message::success("success".to_owned())
        },

        Err(err) => {

            Message::error("error")
        }
    }
}

pub fn get_record(req: HttpRequest<AppState>, query_work_record: Form<QueryWorkRecord>) -> MessageResult<String> {

    let conn = &req.state().conn;

    match query_work_record.query(conn) {

        Ok(data) => {

            Message::success("success".to_owned())
        },

        Err(err) => {

            Message::error("error")
        }
    }
}

pub fn delete(req: HttpRequest<AppState>, delete_work_record: Form<DeleteWorkRecord>) -> MessageResult<String> {

    let conn = &req.state().conn;

    match delete_work_record.delete(conn) {

        Ok(data) => {

            Message::success("success".to_owned())
        },

        Err(err) => {

            Message::error("error")
        }
    }
}
