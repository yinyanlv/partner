use actix_web::error;
use diesel;
use diesel::prelude::*;
use diesel::prelude::MysqlConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use chrono::{Local, NaiveDateTime};

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
    pub fn validate(&self, conn: &Conn) -> Result<RawUser, bool> {

        use common::schema::user::dsl::*;

        let cur_user = User::get_user(conn, &*self.username);

        match cur_user {

            Ok(data) => {
                let cur_password = md5_encode(&*format!("{}{}", self.password.clone(), &*data.salt));

                if cur_password == data.password {
                    Ok(data)
                } else {
                    Err(false)
                }
            },
            Err(_) => {
                Err(false)
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUser {
    pub username: String,
    pub nickname: String,
    pub email: String,
    pub phone: String
}

impl UpdateUser {

    pub fn update(&self, conn: &Conn) -> QueryResult<RawUser> {
        use common::schema::user::dsl::*;

        diesel::update(user.filter(username.eq(&self.username)))
            .set((
                nickname.eq(self.nickname.clone()),
                email.eq(self.email.clone()),
                phone.eq(self.phone.clone())
            ))
            .execute(conn);

        User::get_user(conn, &*self.username)
    }

    pub fn is_email_updateable(&self, conn: &Conn) -> bool {
        use common::schema::user::dsl::*;

        !user
            .filter(username.ne(&self.username).and(email.eq(&self.email)))
            .get_result::<RawUser>(conn).is_ok()
    }

    pub fn is_phone_updateable(&self, conn: &Conn) -> bool {
        use common::schema::user::dsl::*;

        if self.phone == "" {
            return true;
        }

        !user
            .filter(username.ne(&self.username).and(phone.eq(&self.phone)))
            .get_result::<RawUser>(conn).is_ok()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteUser {
    pub username: String
}

impl DeleteUser {

    pub fn delete(&self, conn: &Conn) -> QueryResult<usize> {

        use common::schema::user::dsl::*;

        diesel::delete(user.filter(username.eq(&self.username))).execute(conn)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModifyPasswordUser {
    pub username: String,
    pub password: String,
    pub new_password: String,
    pub confirm_new_password: String
}

impl ModifyPasswordUser {

    pub fn validate(&self, conn: &Conn) -> Result<RawUser, bool> {

        use common::schema::user::dsl::*;

        let cur_user = User::get_user(conn, &*self.username);

        match cur_user {

            Ok(data) => {
                let cur_password = md5_encode(&*format!("{}{}", self.password.clone(), &*data.salt));

                if cur_password == data.password {
                    Ok(data)
                } else {
                    Err(false)
                }
            },
            Err(_) => {
                Err(false)
            }
        }
    }

    pub fn modify_password(&self, conn: &Conn) -> QueryResult<usize> {

        use common::schema::user::dsl::*;
        
        let new_salt = random_string(8);
        let new_password = md5_encode(&*format!("{}{}", self.new_password.clone(), &*new_salt));

        diesel::update(user.filter(username.eq(&self.username)))
               .set((
                    password.eq(new_password),
                    salt.eq(new_salt)
                ))
                .execute(conn)
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

    pub fn is_user_exist(conn: &Conn, cur_username: &str) -> bool {

        use common::schema::user::dsl::*;

        User::get_user(conn, cur_username).is_ok()
    }

    pub fn get_user(conn: &Conn, cur_username: &str) -> QueryResult<RawUser> {

        use common::schema::user::dsl::*;

        user.filter(username.eq(cur_username)).get_result::<RawUser>(conn)
    }

    pub fn is_email_exist(conn: &Conn, cur_email: &str) -> bool {

        use common::schema::user::dsl::*;

        user.filter(email.eq(cur_email)).get_result::<RawUser>(conn).is_ok()
    }

     pub fn is_phone_exist(conn: &Conn, cur_phone: &str) -> bool {

        use common::schema::user::dsl::*;

        user.filter(phone.eq(cur_phone)).get_result::<RawUser>(conn).is_ok()
    }
}
