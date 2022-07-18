use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::env;

pub async fn database_connection() -> Result<PgPool, Box<dyn std::error::Error>>
{
    let uri = env::var("DATABASE_URI")?;
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&uri)
        .await?;

    Ok(pool)
}
