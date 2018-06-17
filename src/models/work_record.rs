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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateWorkRecord {
    pub id: i32,
    pub username: String,
    pub day: NaiveDate,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkRecordEvents {
    pub id: i32,
    pub username: String,
    pub day: NaiveDate,
    pub overtime: Option<f32>,
    pub create_time: NaiveDateTime,
    pub update_time: NaiveDateTime,
    pub events: Vec<RawWorkEvent>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryWorkRecord {
    pub username: String,
    pub day: NaiveDate
}

impl QueryWorkRecord {
    
    pub fn query(&self, conn: &Conn) -> QueryResult<WorkRecordEvents> {

        use common::schema::work_record::dsl::*;

        let record = work_record
                        .filter(
                            username.eq(&self.username)
                                    .and(day.eq(&self.day))
                                )
                        .get_result::<RawWorkRecord>(conn).unwrap();
        let cur_id = record.id;
        let events = QueryWorkEvents::query(conn, cur_id).unwrap();

        Ok(WorkRecordEvents {
                id: record.id,
                username: record.username,
                day: record.day,
                overtime: record.overtime,
                create_time: record.create_time,
                update_time: record.update_time,
                events: events
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryMonthWorkRecord {
    pub username: String, 
    pub year: i32,
    pub month: u32
}

impl QueryMonthWorkRecord {

    pub fn query(&self, conn: &Conn) -> QueryResult<Vec<WorkRecordEvents>> {

        use common::schema::work_record::dsl::*;

        let first_day = NaiveDate::from_ymd(self.year, self.month, 1);
        let last_day = NaiveDate::from_ymd(self.year, self.month, QueryMonthWorkRecord::last_day_of_month(self.year, self.month));

        let mut list = vec![];

        let records = work_record
                        .filter(
                            username.eq(&self.username)
                                    .and(day.ge(first_day))
                                    .and(day.le(last_day))
                                )
                        .load::<RawWorkRecord>(conn).unwrap();

        for record in records {

            let cur_id = record.id;
            let events = QueryWorkEvents::query(conn, cur_id).unwrap();

            list.push(WorkRecordEvents {
                id: record.id,
                username: record.username,
                day: record.day,
                overtime: record.overtime,
                create_time: record.create_time,
                update_time: record.update_time,
                events: events
            });
        }

        Ok(list)
    }

    fn last_day_of_month(year: i32, month: u32) -> u32 {

        NaiveDate::from_ymd_opt(year, month + 1, 1).unwrap_or(NaiveDate::from_ymd(year + 1, 1, 1)).pred().day()
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

        let num = diesel::delete(work_record.filter(id.eq(&self.record_id))).execute(conn).unwrap();
        DeleteWorkEvents.delete(conn, self.record_id);

        Ok(num)
    }
}