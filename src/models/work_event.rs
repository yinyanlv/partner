use chrono::{Local, NaiveDateTime};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkEvent {
    pub username: String,
    pub record_id: i32,
    pub state_time: NaiveDateTime,
    pub end_time: NaiveDateTime
}