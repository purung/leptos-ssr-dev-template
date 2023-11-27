
use anyhow::Result;
use chrono::Local;
use sqlx::postgres::PgPoolOptions;
use ulid::Ulid;

#[tokio::main]
async fn main() -> Result<()> {
    let many_names  = vec!["Inigo Montoya", "Darth Vader", "Emperor"];
    let mut names = many_names.iter().cycle();
    let lines = vec!["Hello. My name is Inigo Montoya. You killed my father! Prepare to die.", "No, Luke, I am your father.", "Now, young Skywalker, you will die."];
    let mut special = lines.iter().cycle();
    let nos = vec!["07234987234", "4673409229"];
    let mut tel = nos.iter().cycle();

    let pool = PgPoolOptions::new().connect(env!("DATABASE_URL")).await?;

    for _ in 0..15 {
        let ulid = Ulid::new().to_string();
        let now = Local::now();
        sqlx::query(
            r#"
            insert into contact_request (uuid, name, tel, special, timestamp)
            values ($1, $2, $3, $4, $5)
            "#,
        )
        .bind(ulid)
        .bind(names.next().unwrap())
        .bind(tel.next().unwrap())
        .bind(special.next().unwrap())
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
