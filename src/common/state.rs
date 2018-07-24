use std::sync::Arc;
use actix::*;
use actix::prelude::*;
use actix_redis::{Command, RedisActor};
use diesel::prelude::MysqlConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use common::lazy_static::CONFIG;
use futures::Future;

pub struct AppState {
    pub conn: PooledConnection<ConnectionManager<MysqlConnection>>,
    pub redis_addr: Addr<RedisActor>
}

impl AppState {

    pub fn new(addr: &str) -> AppState {

        let manager = ConnectionManager::<MysqlConnection>::new(&*CONFIG.mysql.url);

        AppState {
            conn: Pool::builder().build(manager).unwrap().get().expect("can't build mysql pool"),
            redis_addr: RedisActor::start(addr)
        }
    }
}
