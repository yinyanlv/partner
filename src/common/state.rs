use actix::*;
use std::sync::Arc;
use dotenv;
use diesel::prelude::MysqlConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};

pub struct AppState {
    pub conn: PooledConnection<ConnectionManager<MysqlConnection>>
}

impl AppState {

    pub fn new() -> AppState {

        let db_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");
        let manager = ConnectionManager::<MysqlConnection>::new(db_url);

        AppState {
            conn: Pool::builder().build(manager).unwrap().get().expect("can't build mysql pool")
        }
    }
}
