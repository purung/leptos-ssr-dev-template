use async_trait::async_trait;
use chrono::{DateTime, Local};
use leptos::use_context;
use sqlx::PgPool;
use ulid::Ulid;

use super::{errors::EyeError, please::Communicate, Contact};

#[derive(sqlx::FromRow, Debug)]
struct PgCard {
    uuid: String,
    name: String,
    tel: String,
    special: String,
    timestamp: DateTime<Local>,
}

#[async_trait]
impl Communicate<Contact, PgPool> for Contact {
    async fn power() -> Result<PgPool, EyeError> {
        let ctx = use_context::<PgPool>().ok_or(EyeError::ConfigError)?;
        Ok(ctx)
    }
    async fn birth(&self) -> Result<(), EyeError> {
        sqlx::query(
            r#"
            insert into contact_request (uuid, name, tel, special, timestamp)
            values ($1, $2, $3, $4, $5)
        "#,
        )
        .bind(&self.stamp.to_string())
        .bind(&self.name)
        .bind(&self.tel)
        .bind(&self.special)
        .bind(self.timestamp)
        .execute(&Self::power().await?)
        .await?;
        Ok(())
    }

    async fn destroy(ulid: Ulid) -> Result<(), EyeError> {
        sqlx::query(
            r#"
            delete from contact_request where uuid = $1
        "#,
        )
        .bind(&ulid.to_string())
        .execute(&Self::power().await?)
        .await.unwrap();
        Ok(())
    }

    async fn all() -> Result<Vec<Contact>, EyeError> {
        log::info!("Grabbing all cards");
        let rows = sqlx::query_as::<_, PgCard>("select * from contact_request")
            .fetch_all(&Self::power().await?)
            .await?
            .into_iter()
            .map(Into::into)
            .collect::<Vec<Contact>>();
        log::info!("Got {} rows", rows.len());
        Ok(rows)
    }
}
impl From<PgCard> for Contact {
    fn from(val: PgCard) -> Self {
        Contact {
            stamp: Ulid::from_string(&val.uuid).unwrap_or_default(),
            name: val.name,
            tel: val.tel,
            special: val.special,
            timestamp: val.timestamp,
        }
    }
}
