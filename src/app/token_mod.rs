use std::ops::DerefMut;

use anyhow::Context;
use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};

use chrono::Datelike;
use once_cell::sync::OnceCell;
use pasetors::{keys::SymmetricKey, version4::V4};
use serde::Deserialize;
use sqlx::SqlitePool;

use ulid::Ulid;
use validator::Validate;

pub mod confirm;
mod error;

use crate::{
    email::{
        dispatch, ClickLinkMessage, EmailCommon, EmailParts, KnowsHowToRender, Link,
        LoginConfirmation,
    },
    Error, Result, CONFIG,
};

use self::confirm::User;

const COOKIE_NAME: &str = "passauth";

pub fn routes() -> Router<SqlitePool> {
    Router::new()
        .route("/", get(ego))
        .route("/request", post(request_ticket_for))
        .route("/confirm/:ulid", post(confirm::confirm_login_for))
}

#[derive(Debug, Deserialize, Validate)]
pub struct RequestingLogin {
    #[validate(email)]
    email: String,
}

pub async fn ego(user: User) -> Result<Json<User>> {
    Ok(Json(user))
}

pub async fn request_ticket_for(
    State(pool): State<SqlitePool>,
    Json(person): Json<RequestingLogin>,
) -> Result<StatusCode> {
    // Det här möjliga felet bör fångas redan i UI
    person.validate()?;

    let found_person = find_membership_for(person, &pool).await?;

    if let Some(member) = found_person {
        let mut tx = pool.begin().await?;
        let uuid = Ulid::new();
        create_login_token_for(&member, &uuid, &mut tx).await?;
        let email = write_confirmation_email(member, uuid)?;
        dispatch(email).await?;
        tx.commit().await?;
        tracing::info!("Sent login email...");
    }

    Ok(StatusCode::OK)
}

fn write_confirmation_email(member: Member, uuid: Ulid) -> Result<EmailParts> {
    let meta = EmailCommon::builder()
        .fname(&member.fname)
        .lname(&member.lname)
        .email(&member.email)
        .subject(String::from("Inloggning för Istdp Sverige"))
        .build();
    let link = Link::<LoginConfirmation>::new(&uuid).point();
    let email = ClickLinkMessage::builder()
        .metadata(meta)
        .heading("Bekräfta din identitet".to_owned())
        .message("Klicka på länken inom 24 timmar för att logga in.".to_owned())
        .link(link)
        .build();
    email
        .rendered_with_backup()
        .map_err(Error::TemplateError)
}

async fn create_login_token_for(
    member: &Member,
    uuid: &Ulid,
    tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
) -> Result<(), Error> {
    let uuid = uuid.to_string();
    sqlx::query!(
        r#"
                insert into membership_confirmation 
                (email, fname, lname, uuid, year)
                values (?, ?, ?, ?, ?)
            "#,
        member.email,
        member.fname,
        member.lname,
        uuid,
        member.valid
    )
    .execute(tx.deref_mut())
    .await?;
    Ok(())
}

struct Member {
    email: String,
    fname: String,
    lname: String,
    valid: i64,
}

async fn find_membership_for(
    person: RequestingLogin,
    pool: &sqlx::Pool<sqlx::Sqlite>,
) -> Result<Option<Member>, Error> {
    let current_year = chrono::offset::Utc::now().year();
    let person = sqlx::query_as!(
        Member,
        r#"
            select email, fname, lname, valid from member
            where ? <= valid
            and email = ?
            order by valid desc
        "#,
        current_year,
        person.email,
    )
    .fetch_optional(pool)
    .await?;
    Ok(person)
}

static SYMMETRIC_KEY: OnceCell<SymmetricKey<V4>> = OnceCell::new();

fn symmetric_key_please() -> Result<&'static SymmetricKey<V4>> {
    SYMMETRIC_KEY.get_or_try_init(|| -> Result<SymmetricKey<V4>> {
        let symmetric_key = CONFIG
            .get()
            .context("Config init failure")?
            .token_key
            .as_str();
        Ok(SymmetricKey::<V4>::try_from(symmetric_key)
            .context("Symmetric key initialisation error")?)
    })
}
