table! {
    user (id) {
        id -> Integer,
        username -> Varchar,
        nickname -> Nullable<Varchar>,
        email -> Varchar,
        phone -> Nullable<Varchar>,
        role -> Nullable<Tinyint>,
        password -> Varchar,
        salt -> Varchar,
        create_time -> Datetime,
        update_time -> Datetime,
    }
}

table! {
    work_event (id) {
        id -> Integer,
        record_id -> Integer,
        start_time -> Datetime,
        end_time -> Datetime,
        create_time -> Datetime,
        update_time -> Datetime,
    }
}

table! {
    work_record (id) {
        id -> Integer,
        username -> Varchar,
        day -> Date,
        overtime -> Nullable<Float>,
        create_time -> Datetime,
        update_time -> Datetime,
    }
}

joinable!(work_event -> work_record (record_id));

allow_tables_to_appear_in_same_query!(
    user,
    work_event,
    work_record,
);
