use diesel;
use diesel::prelude::*;
use diesel::expression::sql_literal::sql;
use diesel::prelude::MysqlConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use chrono::{Local, NaiveDate, NaiveDateTime};

use models::work_event::*;
use common::schema::work_record;

type Conn = PooledConnection<ConnectionManager<MysqlConnection>>;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct RawWorkRecord {
    pub id: i32,
    pub username: String,
    pub day: NaiveDate,
    pub overtime: Option<f32>,
    pub create_time: NaiveDateTime,
    pub update_time: NaiveDateTime
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWorkRecord {
    pub username: String,
    pub day: NaiveDate,
    pub overtime: f32,
    pub events: Vec<CreateWorkEvent>
}

impl CreateWorkRecord {

    pub fn into_work_record(&self) -> WorkRecord {

        WorkRecord {
            username: self.username.clone(),
            day: self.day.clone(),
            overtime: self.overtime,
            create_time: Local::now().naive_utc(),
            update_time: Local::now().naive_utc()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[table_name="work_record"]
pub struct WorkRecord {
    pub username: String,
    pub day: NaiveDate,
    pub overtime: f32,
    pub create_time: NaiveDateTime,
    pub update_time: NaiveDateTime
}

impl WorkRecord {

    pub fn create(&self, conn: &Conn, events: &Vec<CreateWorkEvent>) -> Result<i32, String> {
        use common::schema::work_record::dsl::*;

        diesel::insert_into(work_record).values(self).execute(conn);

        let last_insert_id = sql("SELECT LAST_INSERT_ID()").get_result(conn).unwrap();
        let mut new_events = vec![];
        let now = Local::now().naive_utc();

        for event in events {
            
            new_events.push(event.into_work_event(last_insert_id, &now));
        }

        if new_events.len() > 0 {
        
            WorkEvent::create(conn, &new_events);
        }

        Ok(last_insert_id)
    }

    pub fn update(&self, conn: &Conn) {

    }

    pub fn query(conn: &Conn, username: &str, day: NaiveDate) {

    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteWorkRecord {
    pub username: String,
    pub record_id: i32
}

impl DeleteWorkRecord {

    pub fn delete(&self, conn: &Conn) -> QueryResult<usize> {
        use common::schema::work_record::dsl::*;

        diesel::delete(work_record.filter(id.eq(&self.record_id))).execute(conn);
        DeleteEvents.delete(conn, self.record_id);

        Ok(1)
    }
}