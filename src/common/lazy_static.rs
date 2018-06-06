use dotenv;
use diesel::prelude::MysqlConnection;
use diesel::r2d2::{ConnectionManager, Pool, ManageConnection};

lazy_static! {

    pub static ref POOL: Pool<ManageConnection, Error>= {

        let db_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");
        let manager = ConnectionManager::<MysqlConnection>::new(db_url);

        Pool::builder().build(manager).expect("failed to build mysql pool")

    };
}
