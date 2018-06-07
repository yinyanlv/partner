use std::sync::Arc;
use actix_web::{HttpRequest, Responder};
use chrono::{Local, NaiveDateTime};

use common::state::AppState;
use models::user::User;

pub fn register(req: HttpRequest<AppState>) -> impl Responder {

    let conn = &req.state().conn;

    let user = User {
        username: "test".to_owned(),
        nickname: "aaa".to_owned(),
        email: "a@qq.com".to_owned(),
        phone: "123334444444".to_owned(),
        password: "111".to_owned(),
        salt: "md5".to_owned(),
        create_time: Local::now().naive_utc(),
        update_time: Local::now().naive_utc()
    };

    let res = match user.create(conn) {
        Ok(msg) => msg,
        Err(msg) => msg
    };

    res
}

pub fn login(req: HttpRequest<AppState>) -> impl Responder {

    "login"
}

pub fn update(req: HttpRequest<AppState>) -> impl Responder {

    "update"
}

pub fn reset(req: HttpRequest<AppState>) -> impl Responder {

    "reset"
}