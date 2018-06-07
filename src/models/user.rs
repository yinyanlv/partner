use actix_web::error;
use diesel;
use diesel::prelude::*;
use chrono::{Local, NaiveDateTime};
use diesel::prelude::MysqlConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};

use common::schema::user;
use common::schema::user::dsl::*;

type Conn = PooledConnection<ConnectionManager<MysqlConnection>>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterUser {
    pub username: String,
    pub nickname: String,
    pub email: String,
    pub phone: String,
    pub password: String
}

impl RegisterUser {

    pub fn into_user(&self) -> User {

        User {
            username: self.username.clone(),
            nickname: self.nickname.clone(),
            email: self.email.clone(),
            phone: self.phone.clone(),
            password: self.password.clone(),
            salt: "md5".to_owned(),
            create_time: Local::now().naive_utc(),
            update_time: Local::now().naive_utc()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginUser {
    pub username: String,
    pub password: String,
    pub remember: bool,
}

impl LoginUser {
    pub fn validate(&self) -> Result<String, String> {

        
        Ok("login".to_owned())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUser {
    pub username: String,
    pub nickname: String,
    pub email: String,
    pub phone: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteUser {
    pub username: String
}

impl DeleteUser {

    pub fn delete(&self, conn: &Conn) -> Result<String, String> {

        diesel::delete(user.filter(username.eq(&self.username))).execute(conn).expect("delete user error");

        Ok("delete success".to_owned())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[table_name="user"]
pub struct User {
    pub username: String,
    pub nickname: String,
    pub email: String,
    pub phone: String,
    pub password: String,
    pub salt: String,
    pub create_time: NaiveDateTime,
    pub update_time: NaiveDateTime
}

impl User {
    pub fn create(&self, conn: &Conn) -> Result<String, String> {

        diesel::insert_into(user).values(self).execute(conn).expect("create user error");
        Ok("create success".to_string())
    }

    pub fn update(&self) -> Result<String, String> {

        Ok("update success".to_owned())
    }
}
