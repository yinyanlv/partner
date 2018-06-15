use diesel;
use diesel::prelude::*;
use chrono::{Local, NaiveDate};
use diesel::prelude::MysqlConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};

use super::work_event::*;

type Conn = PooledConnection<ConnectionManager<MysqlConnection>>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawWorkRecord {
    
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkRecord {
    pub username: String,
    pub day: NaiveDate,
    pub overtime: f32,
    pub events: Vec<WorkEvent>
}

impl WorkRecord {

    pub fn create(&self, conn: &Conn) {

    }

    pub fn update(&self, conn: &Conn) {

    }

    pub fn query(conn: &Conn, username: &str, day: NaiveDate) {

    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteWorkRecord {
    pub record_id: i32
}

impl DeleteWorkRecord {
    pub fn delete(&self, conn: &Conn) {

    }
}


