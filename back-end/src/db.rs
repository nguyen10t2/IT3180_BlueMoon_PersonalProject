use sqlx::{Pool, Postgres};
use std::env;
use dotenvy::dotenv;

pub type DbPool = Pool<Postgres>;

pub async fn init_db() -> DbPool {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL chưa được khai báo");
    sqlx::PgPool::connect(&db_url)
        .await
        .expect("Không thể kết nối tới database")
}