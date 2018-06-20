use diesel;
use diesel::prelude::*;
use diesel::expression::sql_literal::sql;
use diesel::prelude::MysqlConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use chrono::prelude::*;
use chrono::{Local, NaiveDate, NaiveDateTime};

use models::work_event::*;
use common::schema::work_record;

type Conn = PooledConnection<ConnectionManager<MysqlConnection>>;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct RawWorkRecord {
    pub id: i32,
    pub username: String,
    pub date: NaiveDateTime,
    pub overtime: Option<f32>,
    pub create_time: NaiveDateTime,
    pub update_time: NaiveDateTime
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWorkRecord {
    pub username: String,
    pub date: DateTime<Utc>,
    pub overtime: f32,
    pub events: Vec<CreateWorkEvent>
}

impl CreateWorkRecord {

    pub fn into_work_record(&self) -> WorkRecord {

        WorkRecord {
            username: self.username.clone(),
            date: self.date.naive_utc(),
            overtime: self.overtime,
            create_time: Local::now().naive_utc(),
            update_time: Local::now().naive_utc()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateWorkRecord {
    pub id: i32,
    pub username: String,
    pub overtime: f32,
    pub events: Vec<CreateWorkEvent>
}

impl UpdateWorkRecord {

    pub fn update(&self, conn: &Conn, events: &Vec<CreateWorkEvent>) -> QueryResult<usize> {

        use common::schema::work_record::dsl::*;

        if events.len() == 0 && self.overtime == 0.0 {

            let delete_record = DeleteWorkRecord {
                username: self.username.clone(),
                record_id: self.id
            };
            
            return delete_record.delete(conn);
        }

        let num = diesel::update(work_record.filter(id.eq(self.id)))
                            .set((
                                overtime.eq(self.overtime),
                                update_time.eq(Local::now().naive_utc())
                            ))
                            .execute(conn)
                            .unwrap();
        
        DeleteWorkEvents.delete(conn, self.id);

        if events.len() > 0 {

            let mut new_events = vec![];
            let now = Local::now().naive_utc();

            for event in events {
                
                new_events.push(event.into_work_event(self.id, &now));
            }

            if new_events.len() > 0 {
            
                WorkEvent::create(conn, &new_events);
            }
        }

        Ok(num)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[table_name="work_record"]
pub struct WorkRecord {
    pub username: String,
    pub date: NaiveDateTime,
    pub overtime: f32,
    pub create_time: NaiveDateTime,
    pub update_time: NaiveDateTime
}

impl WorkRecord {

    pub fn create(&self, conn: &Conn, events: &Vec<CreateWorkEvent>) -> QueryResult<usize> {
        use common::schema::work_record::dsl::*;

        let res = diesel::insert_into(work_record).values(self).execute(conn);

        if res.is_err() {

            return res; 
        }

        let last_insert_id = sql("SELECT LAST_INSERT_ID()").get_result(conn).unwrap();
        let mut new_events = vec![];
        let now = Local::now().naive_utc();

        for event in events {
            
            new_events.push(event.into_work_event(last_insert_id, &now));
        }

        if new_events.len() > 0 {
        
            WorkEvent::create(conn, &new_events);
        }

        Ok(last_insert_id as usize)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkRecordResponse {
    pub id: i32,
    pub username: String,
    pub date: NaiveDateTime,
    pub overtime: Option<f32>,
    pub create_time: NaiveDateTime,
    pub update_time: NaiveDateTime,
    pub events: Vec<RawWorkEvent>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryWorkRecord {
    pub username: String,
    pub date: DateTime<Utc>
}

impl QueryWorkRecord {
    
    pub fn query(&self, conn: &Conn) -> QueryResult<WorkRecordResponse> {

        use common::schema::work_record::dsl::*;

        let record = work_record
                        .filter(username.eq(&self.username))
                        .filter(date.eq(&self.date.naive_utc()))
                        .get_result::<RawWorkRecord>(conn).unwrap();
        let cur_id = record.id;
        let events = QueryWorkEvents::query(conn, cur_id).unwrap();

        Ok(WorkRecordResponse {
                id: record.id,
                username: record.username,
                date: record.date,
                overtime: record.overtime,
                create_time: record.create_time,
                update_time: record.update_time,
                events: events
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryMonthWorkRecord {
    pub username: String, 
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>
}

impl QueryMonthWorkRecord {

    pub fn query(&self, conn: &Conn) -> QueryResult<Vec<WorkRecordResponse>> {

        use common::schema::work_record::dsl::*;

        let mut list = vec![];

        let records = work_record
                        .filter(username.eq(&self.username))
                        .filter(date.ge(&self.start_date.naive_utc()))
                        .filter(date.le(&self.end_date.naive_utc()))
                        .load::<RawWorkRecord>(conn).unwrap();

        for record in records {

            let cur_id = record.id;
            let events = QueryWorkEvents::query(conn, cur_id).unwrap();

            list.push(WorkRecordResponse {
                id: record.id,
                username: record.username,
                date: record.date,
                overtime: record.overtime,
                create_time: record.create_time,
                update_time: record.update_time,
                events: events
            });
        }

        Ok(list)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteWorkRecord {
    pub username: String,
    pub record_id: i32
}

impl DeleteWorkRecord {

    pub fn delete(&self, conn: &Conn) -> QueryResult<usize> {
        use common::schema::work_record::dsl::*;

        let res = DeleteWorkEvents.delete(conn, self.record_id);

        if res.is_err() {
            return res;
        }

        diesel::delete(work_record.filter(id.eq(&self.record_id))).execute(conn)
    }
}