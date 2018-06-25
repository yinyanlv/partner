use actix_web::{HttpRequest, Json};
use actix_web::middleware::session::RequestSession;
use actix_redis::{Command, RedisActor};

use common::state::AppState;
use common::util::is_unauthorized;
use common::lazy_static::CONFIG;
use models::user::*;
use models::response::{Message, MessageResult};

pub fn register(req: HttpRequest<AppState>, register_user: Json<RegisterUser>) -> MessageResult<String> {

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

pub fn login(req: HttpRequest<AppState>, login_user: Json<LoginUser>) -> MessageResult<RawUser> {

    let conn = &req.state().conn;

    match login_user.validate(conn) {

        Ok(data) => {

            req.session().set::<RawUser>("user", data.clone());

            if login_user.remember {
                req.session().set::<bool>("remember", true);
            } else {
                req.session().set::<bool>("remember", false);
            }

            Message::success(data)
        },
        Err(err) => Message::error("用户名或密码错误")
    }
}

pub fn logout(req: HttpRequest<AppState>) -> MessageResult<String> {

    if is_unauthorized(&req) {
        
        return Message::error("Unauthorized");
    }

    req.session().clear();

    Message::success("退出登录成功".to_owned())
}

pub fn update(req: HttpRequest<AppState>, update_user: Json<UpdateUser>) -> MessageResult<RawUser> {

    if is_unauthorized(&req) {
        
        return Message::error("Unauthorized");
    }

    let conn = &req.state().conn;

    if !update_user.is_email_updateable(conn) {

        return Message::error("该邮箱已被注册");
    }

    if !update_user.is_phone_updateable(conn) {

         return Message::error("该手机号已被绑定");
    }

    match update_user.update(conn) {
        Ok(data) => Message::success(data),
        Err(err) => Message::error(&*err.to_string())
    }
}

pub fn delete(req: HttpRequest<AppState>, delete_user: Json<DeleteUser>) -> MessageResult<String> {

    if is_unauthorized(&req) {
        
        return Message::error("Unauthorized");
    }

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

pub fn modify_password(req: HttpRequest<AppState>, modify_password_user: Json<ModifyPasswordUser>) -> MessageResult<String> {

    if is_unauthorized(&req) {
        
        return Message::error("Unauthorized");
    }

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