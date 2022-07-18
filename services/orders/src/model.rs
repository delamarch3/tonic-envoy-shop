use crate::rpc::Order;
use sqlx::{PgPool, Row};

impl<'r> sqlx::FromRow<'r, sqlx::postgres::PgRow> for Order {
    fn from_row(row: &'r sqlx::postgres::PgRow) -> Result<Self, sqlx::Error> {
        let id = row.try_get("id")?;
        let userid = row.try_get("userid")?;
        let product = row.try_get("product")?;
        let total = row.try_get("total")?;

        Ok(Order {
            id,
            userid,
            product,
            total,
        })
    }
}

impl Order {
    pub async fn get_by_userid(
        pool: PgPool,
        userid: i64,
    ) -> Result<Vec<Self>, Box<dyn std::error::Error>> {
        let orders = sqlx::query_as(
            r#"
            SELECT * FROM orders WHERE userid = $1   
            "#,
        )
        .bind(userid)
        .fetch_all(&pool)
        .await?;

        Ok(orders)
    }

    pub async fn add_user_total(
        pool: PgPool,
        userid: i64,
    ) -> Result<f32, sqlx::Error> {
        let total = sqlx::query(
            r#"
            SELECT COALESCE(SUM(total),0) AS total FROM orders WHERE userid = $1
            "#,
        )
        .bind(userid)
        .fetch_one(&pool)
        .await?;

        let total = total.try_get("total")?;
        Ok(total)
    }

    pub async fn create(
        pool: PgPool,
        userid: i64,
        product: String,
        total: f32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        sqlx::query(
            r#"
            INSERT INTO orders (userid, product, total) VALUES ($1, $2, $3)
            "#,
        )
        .bind(userid)
        .bind(product)
        .bind(total)
        .execute(&pool)
        .await?;
        Ok(())
    }
}
