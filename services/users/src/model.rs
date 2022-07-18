use sqlx::{PgPool, Row};

#[derive(sqlx::FromRow, Debug)]
pub struct User {
    pub id: i64,
    pub firstname: String,
    pub lastname: String,
    pub credit_limit: i64,
}

impl User {
    pub async fn get(
        pool: PgPool,
        id: i64,
    ) -> Result<User, Box<dyn std::error::Error>> {
        let user = sqlx::query_as(
            r#"
            SELECT * FROM users WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_one(&pool)
        .await?;

        Ok(user)
    }

    pub async fn get_credit_limit(
        pool: PgPool,
        id: i64,
    ) -> Result<i64, Box<dyn std::error::Error>> {
        let credit_limit = sqlx::query(
            r#"
            SELECT credit_limit FROM users WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_one(&pool)
        .await?;
        let credit_limit = credit_limit.try_get("credit_limit")?;
        Ok(credit_limit)
    }
}
