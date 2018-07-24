use actix_web::{HttpRequest, State, Json, Query, FromRequest};

use common::state::AppState;
use models::work_record::*;
use models::response::{Message, MessageResult};

pub fn create((state, create_work_record): (State<AppState>, Json<CreateWorkRecord>)) -> MessageResult<usize> {

    let conn = &state.conn;
    let work_record = create_work_record.into_work_record();

    match work_record.create(conn, &create_work_record.events) {

        Ok(data) => {

            Message::success(data)
        },

        Err(err) => {

            Message::error(&*err.to_string())
        }
    }
}

pub fn update((state, update_work_record): (State<AppState>, Json<UpdateWorkRecord>)) -> MessageResult<usize> {

    let conn = &state.conn;

    match update_work_record.update(conn, &update_work_record.events) {

        Ok(data) => {

            Message::success(data)
        },

        Err(err) => {

            Message::error(&*err.to_string())
        }
    }
}

pub fn get_records((state, query_month_work_record): (State<AppState>, Json<QueryMonthWorkRecord>)) -> MessageResult<Vec<WorkRecordResponse>> {

    let conn = &state.conn;

    match query_month_work_record.query(conn) {

        Ok(data) => {

            Message::success(data)
        },

        Err(err) => {

            Message::error(&*err.to_string())
        }
    }
}

pub fn get_record((state, query_work_record): (State<AppState>, Json<QueryWorkRecord>)) -> MessageResult<WorkRecordResponse> {

    let conn = &state.conn;

    match query_work_record.query(conn) {

        Ok(data) => {

            Message::success(data)
        },

        Err(err) => {

            Message::error(&*err.to_string())
        }
    }
}

pub fn delete((state, delete_work_record): (State<AppState>, Query<DeleteWorkRecord>)) -> MessageResult<usize> {
    
    let conn = &state.conn;

    match delete_work_record.delete(conn) {

        Ok(data) => {

            Message::success(data)
        },

        Err(err) => {

            Message::error(&*err.to_string())
        }
    }
}
