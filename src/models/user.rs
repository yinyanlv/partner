use actix_web::error;
use diesel;
use diesel::prelude::*;
use chrono::{Local, NaiveDateTime};
use diesel::prelude::MysqlConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};

use common::schema::user;

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
    pub fn create(&self, conn: &PooledConnection<ConnectionManager<MysqlConnection>>) -> Result<String, String> {
        use common::schema::user::dsl::*;

        diesel::insert_into(user).values(self).execute(conn);
        Ok("success".to_string())
    }

    pub fn update(&self) -> Result<String, String> {

        Ok("update".to_owned())

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
