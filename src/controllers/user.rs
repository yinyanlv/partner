use actix_web::{HttpRequest, Json, State};
use actix_web::middleware::session::RequestSession;

use common::state::AppState;
use models::user::*;
use models::response::{Message, MessageResult};

pub fn register((state, register_user): (State<AppState>, Json<RegisterUser>)) -> MessageResult<String> {

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

pub fn login((req, login_user): (HttpRequest<AppState>, Json<LoginUser>)) -> MessageResult<RawUser> {

    let conn = &req.state().conn;

    match login_user.validate(conn) {

        Ok(data) => {

            req.session().set::<RawUser>("user", data.clone()).unwrap();

            if login_user.remember {
                req.session().set::<bool>("remember", true).unwrap();
            } else {
                req.session().set::<bool>("remember", false).unwrap();
            }

            Message::success(data)
        },
        Err(_err) => Message::error("用户名或密码错误")
    }
}

pub fn logout(req: &HttpRequest<AppState>) -> MessageResult<String> {

    req.session().clear();

    Message::success("退出登录成功".to_owned())
}

pub fn update((state, update_user): (State<AppState>, Json<UpdateUser>)) -> MessageResult<RawUser> {

    let conn = &state.conn;

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

pub fn delete((state, delete_user): (State<AppState>, Json<DeleteUser>)) -> MessageResult<String> {

    let conn = &state.conn;

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

pub fn modify_password((state, modify_password_user): (State<AppState>, Json<ModifyPasswordUser>)) -> MessageResult<String> {

    let conn = &state.conn;

    if modify_password_user.new_password != modify_password_user.confirm_new_password {

        return Message::error("您两次输入的新密码不一致");
    }    

    match modify_password_user.validate(conn) {

        Ok(_data) => {

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