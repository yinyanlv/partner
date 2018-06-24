use actix::*;
use std::sync::Arc;
use diesel::prelude::MysqlConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use common::lazy_static::CONFIG;

pub struct AppState {
    pub conn: PooledConnection<ConnectionManager<MysqlConnection>>
}

impl AppState {

    pub fn new() -> AppState {

        let manager = ConnectionManager::<MysqlConnection>::new(&*CONFIG.mysql.url);

        AppState {
            conn: Pool::builder().build(manager).unwrap().get().expect("can't build mysql pool")
        }
    }
}
