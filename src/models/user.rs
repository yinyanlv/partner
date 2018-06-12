use actix_web::error;
use diesel;
use diesel::prelude::*;
use chrono::{Local, NaiveDateTime};
use diesel::prelude::MysqlConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};

use common::schema::user;
use common::util::*;

type Conn = PooledConnection<ConnectionManager<MysqlConnection>>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterUser {
    pub username: String,
    pub email: String,
    pub password: String,
    pub confirm_password: String
}

impl RegisterUser {

    pub fn into_user(&self) -> User {

        let salt = random_string(8);
        let password = md5_encode(&*format!("{}{}", self.password.clone(), &*salt));

        User {
            username: self.username.clone(),
            nickname: "".to_owned(),
            email: self.email.clone(),
            phone: "".to_owned(),
            password: password,
            role: 0,
            salt: salt,
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
    pub fn validate(&self, conn: &Conn) -> Result<bool, bool> {
        
        let is_user_exist = User::is_user_exist(conn, &*self.username);

        if !is_user_exist {
            return Err(false);   
        }

        Ok(true) 
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

        use common::schema::user::dsl::*;

        diesel::delete(user.filter(username.eq(&self.username))).execute(conn);

        Ok("delete success".to_owned())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct RawUser {
    pub id: i32,
    pub username: String,
    pub nickname: Option<String>,
    pub email: String,
    pub phone: Option<String>,
    pub role: Option<i8>,
    pub password: String,
    pub salt: String,
    pub create_time: NaiveDateTime,
    pub update_time: NaiveDateTime
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[table_name="user"]
pub struct User {
    pub username: String,
    pub nickname: String,
    pub email: String,
    pub phone: String,
    pub role: i8,
    pub password: String,
    pub salt: String,
    pub create_time: NaiveDateTime,
    pub update_time: NaiveDateTime
}

impl User {

    pub fn create(&self, conn: &Conn) -> Result<String, String> {

        use common::schema::user::dsl::*;

        let res = diesel::insert_into(user).values(self).execute(conn);

        match res {
            Ok(_) => Ok("".to_owned()),
            Err(err) => Err(err.to_string())
        }
    }

    pub fn update(&self) -> Result<String, String> {

        Ok("update success".to_owned())
    }

    pub fn is_user_exist(conn: &Conn, _username: &str) -> bool {

        use common::schema::user::dsl::*;

        let res = user.filter(username.eq(_username)).get_result::<RawUser>(conn);

        match res {
            Ok(_) => true,
            Err(_) => false
        }
    }

    pub fn is_email_exist(conn: &Conn, _email: &str) -> bool {

        use common::schema::user::dsl::*;

        let res = user.filter(email.eq(_email)).get_result::<RawUser>(conn);

        match res {
            Ok(_) => true,
            Err(_) => false
        }
    }
}
