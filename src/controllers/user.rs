use std::sync::Arc;
use actix_web::{HttpRequest, Responder, State, Json, Form, Result};

use common::state::AppState;
use models::user::*;
use models::response::RegisterMessage;

pub fn register(register_user: Form<RegisterUser>, state: State<AppState>) -> Result<Json<RegisterMessage>> {

    let conn = &state.conn;

    if User::is_user_exist(conn, &*register_user.username) {

        return RegisterMessage {
            success: false,
            message: "该用户名已被注册".to_owned()
        }.respond();
    }

    if User::is_email_exist(conn, &*register_user.email) {

        return RegisterMessage {
            success: false,
            message: "该邮箱已被注册".to_owned()
        }.respond();
    }

    let user = register_user.into_user();

    match user.create(conn) {
        Ok(_) => {

            RegisterMessage {
                success: true,
                message: "".to_owned()
            }.respond()
        },
        Err(err) => {

            RegisterMessage {
                success: false,
                message: err
            }.respond()
        }
    }
}

pub fn login(login_user: Form<LoginUser>, state: State<AppState>) -> impl Responder {

    let conn = &state.conn;

    let res = match login_user.validate(conn) {
        Ok(msg) => msg,
        Err(err) => err
    };

    ""
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