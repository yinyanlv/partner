use std::sync::Arc;
use actix_web::{HttpRequest, Responder, State, Json, Form};

use common::state::AppState;
use models::user::*;

pub fn register(register_user: Form<RegisterUser>, state: State<AppState>) -> impl Responder {

    let conn = &state.conn;
    let user = register_user.into_user();

    let res = match user.create(conn) {
        Ok(msg) => msg,
        Err(err) => err
    };

    res
}

pub fn login(login_user: Form<LoginUser>, state: State<AppState>) -> impl Responder {

    "login"
}

pub fn update(update_user: Form<UpdateUser>, state: State<AppState>) -> impl Responder {

    "update"
}

pub fn delete(delete_user: Form<DeleteUser>, state: State<AppState>) -> impl Responder {

    let conn = &state.conn;
    
    let res = match delete_user.delete(conn) {
        Ok(msg) => msg,
        Err(err) => err
    };

    res
}

pub fn reset_password(req: HttpRequest<AppState>) -> impl Responder {

    "reset"
}