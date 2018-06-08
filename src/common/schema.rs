table! {
    user (id) {
        id -> Integer,
        username -> Varchar,
        nickname -> Nullable<Varchar>,
        email -> Varchar,
        phone -> Nullable<Varchar>,
        role -> Nullable<Unsigned<Tinyint>>,
        password -> Varchar,
        salt -> Varchar,
        create_time -> Datetime,
        update_time -> Datetime,
    }
}