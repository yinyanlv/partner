use actix_web::middleware::session::RequestSession;
use actix_web::{HttpRequest, Responder, State, Form};

use common::state::AppState;
use models::user::*;
use models::response::{Message, MessageResult};

pub fn register(req: HttpRequest<AppState>, register_user: Form<RegisterUser>) -> MessageResult<String> {

    let conn = &req.state().conn;

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

pub fn login(req: HttpRequest<AppState>, login_user: Form<LoginUser>) -> MessageResult<String> {

    let conn = &req.state().conn;

    match login_user.validate(conn) {
        Ok(data) => {

            req.session().set("user", data);
            Message::success("登录成功".to_owned())
        },
        Err(err) => Message::error("用户名或密码错误")
    }
}

pub fn update(req: HttpRequest<AppState>, update_user: Form<UpdateUser>) -> MessageResult<String> {

    let user = req.session().get::<RawUser>("user");

    println!("current user: {:?}", user);

    let conn = &req.state().conn;

    if !update_user.is_email_updateable(conn) {

        return Message::error("该邮箱已被注册");
    }

    if !update_user.is_phone_updateable(conn) {

         return Message::error("该手机号已被绑定");
    }

    match update_user.update(conn) {
        Ok(_) => Message::success("用户信息修改成功".to_owned()),
        Err(err) => Message::error(&*err.to_string())
    }
}

pub fn delete(req: HttpRequest<AppState>, delete_user: Form<DeleteUser>) -> MessageResult<String> {

    let conn = &req.state().conn;
    
    match delete_user.delete(conn) {
        Ok(data) => {

            if data == 0 {
                Message::error("删除用户失败，该用户不存在")
            } else {
                Message::success("删除用户成功".to_owned())
            }
        },
        Err(err) => Message::error(&*err.to_string())
    }
}

pub fn modify_password(req: HttpRequest<AppState>, modify_password_user: Form<ModifyPasswordUser>) -> MessageResult<String> {
    
    let conn = &req.state().conn;

    if modify_password_user.new_password != modify_password_user.confirm_new_password {

        return Message::error("您两次输入的新密码不一致");
    }    

    match modify_password_user.validate(conn) {

        Ok(data) => {

            match modify_password_user.modify_password(conn) {
                Ok(_) => {
                    Message::success("密码修改成功，请重新登录".to_owned())
                },
                Err(err) => {
                    Message::error(&*err.to_string())
                }
            }
        },
        Err(_) => Message::error("您输入的原密码不正确")
    }
}