
use anyhow::Result;
use chrono::Local;
use sqlx::postgres::PgPoolOptions;
use ulid::Ulid;

#[tokio::main]
async fn main() -> Result<()> {
    let name: &'static str = "Inigo Montoya";
    let tel: &'static str = "0707123728";
    let special: &'static str = "Prepare to die";

    let pool = PgPoolOptions::new().connect(env!("DATABASE_URL")).await?;

    for _ in 0..5 {
        let ulid = Ulid::new().to_string();
        let now = Local::now();
        sqlx::query(
            r#"
            insert into contact_request (uuid, name, tel, special, timestamp)
            values ($1, $2, $3, $4, $5)
            "#,
        )
        .bind(ulid)
        .bind(name)
        .bind(tel)
        .bind(special)
        .bind(now)
        .execute(&pool)
        .await?;
    }

    let res = sqlx::query!("select * from contact_request").fetch_all(&pool).await?;
    for r in res {
        println!("{:?}", r);
    }

    Ok(())
}
