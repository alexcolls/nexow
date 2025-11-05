use sqlx::{Pool, Postgres};

pub async fn connect(db_url: &str) -> Pool<Postgres> {
    sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(db_url)
        .await
        .expect("DB connect")
}
