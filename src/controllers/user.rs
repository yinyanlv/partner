use std::sync::Arc;
use actix_web::{HttpRequest, Responder, State, Json, Form, Result};

use common::state::AppState;
use models::user::*;
use models::response::Message;

pub fn register(register_user: Form<RegisterUser>, state: State<AppState>) -> Result<Json<Message<String>>> {

    let conn = &state.conn;

    if register_user.password != register_user.confirm_password {

        return Message::error("两次输入的密码不一致");
    }

    if User::is_user_exist(conn, &*register_user.username) {

        return Message::error("该用户名已被注册");
    }

    if User::is_email_exist(conn, &*register_user.email) {

        return Message::error("该邮箱已被注册");
    }

    let user = register_user.into_user();

    match user.create(conn) {
        Ok(_) => {

            Message::success("".to_owned())
        },
        Err(err) => {

            Message::error(&*err)
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