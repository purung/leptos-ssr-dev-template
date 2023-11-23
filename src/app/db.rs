use async_trait::async_trait;
use chrono::{DateTime, Local};
use leptos::{use_context, ServerFnError};
use sqlx::PgPool;
use ulid::Ulid;

use super::{errors::EyeError, please::Communicate, Contact};

#[derive(sqlx::FromRow, Debug)]
struct PgCard {
    stamp: String,
    name: String,
    tel: String,
    special: String,
    timestamp: DateTime<Local>,
}

#[async_trait]
impl Communicate<Contact, PgPool> for Contact{

    async fn power() -> Result<PgPool, EyeError> {
        Ok(use_context::<PgPool>().ok_or(EyeError::ConfigError)?)
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
        .bind(&self.timestamp)
        .execute(&Self::power().await?)
        .await?;
        Ok(())
    }

    async fn destroy(ulid: Ulid) -> Result<(), EyeError> {
        sqlx::query(
            r#"
            delete from contact_request where uuid = $?
        "#,
        )
        .bind(&ulid.to_string())
        .execute(&Self::power().await?)
        .await?;
        Ok(())
    }

    async fn all() -> Result<Vec<Contact>, EyeError> {
        Ok(sqlx::query_as::<_, PgCard>("select * from contat_request")
            .fetch_all(&Self::power().await?)
            .await?
            .into_iter()
            .map(Into::into)
            .collect())
    }
}
impl Into<Contact> for PgCard {
    fn into(self) -> Contact {
        Contact {
            stamp: Ulid::from_string(&self.stamp).unwrap_or_default(),
            name: self.name,
            tel: self.tel,
            special: self.special,
            timestamp: self.timestamp,
        }
    }
}
