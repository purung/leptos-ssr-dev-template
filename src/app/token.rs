use std::{ops::DerefMut, time::Duration};

use anyhow::Context;
use async_trait::async_trait;
use axum::{extract::FromRequestParts, http::request::Parts, Json};
use pasetors::{
    claims::{Claims, ClaimsValidationRules},
    local,
    token::UntrustedToken,
    version4::V4,
    Local,
};
use serde::{Deserialize, Serialize};

use crate::{Error, Result};

use tower_cookies::{cookie, Cookie, Cookies};

use sqlx::{Pool, Sqlite, SqlitePool, Transaction};

use axum::extract::State;

use ulid::Ulid;

use axum::extract::Path;

use super::error::AuthError;

enum User {
    Maria,
    Admin
}

const WEEK: cookie::time::Duration = cookie::time::Duration::seconds(60 * 60 * 24 * 7);
const COOKIE_NAME: &str = "passauth";
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

pub async fn confirm_login_for(
    Path(ulid): Path<Ulid>,
    State(pool): State<SqlitePool>,
    cookies: Cookies,
) -> Result<Json<User>> {
    tracing::debug!(">>> Getting user with privileges...");
    let user = user_with_privileges(&pool, &ulid).await?;
    let encrypted_user = user.clone().encrypt().context("No good encryption")?;
    tracing::info!(">>> Adding cookie...");
    let mut tx = pool.begin().await?;
    _clean_tokens(ulid, &mut tx).await?;
    let cookie = _bake_cookie(encrypted_user);

    cookies.add(cookie);
    tx.commit().await?;

    Ok(Json(user))
}

fn _bake_cookie<'c>(value: String) -> Cookie<'c> {
    let cookie = Cookie::build(super::COOKIE_NAME, value)
        .max_age(WEEK)
        .expires(week_long_expires())
        .http_only(true)
        .path("/");

    if cfg!(not(feature = "local")) {
        return cookie
            .secure(true)
            .same_site(cookie::SameSite::Strict)
            .finish();
    } else {
        cookie.finish()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    #[serde(alias = "sub")]
    email: String,
    fname: String,
    lname: String,
    privileges: Option<Vec<String>>,
}

impl User {
    pub fn new(
        email: String,
        fname: String,
        lname: String,
        privileges: Option<Vec<String>>,
    ) -> Self {
        Self {
            email,
            fname,
            lname,
            privileges,
        }
    }

    pub fn can(&self, get_away_with: &str) -> Result<(), super::error::AuthError> {
        self.privileges
            .as_ref()
            .map(|p| p.iter().any(|f| f == get_away_with))
            .and(Some(()))
            .ok_or(super::error::AuthError::CannotDoError)
    }

    fn encrypt(self) -> Result<String, super::error::AuthError> {
        let sk = super::symmetric_key_please().context("Can't has symmetric key")?;
        tracing::info!(">>> Generating claims...");
        let claims: Claims = self.try_into().context("Token generation fail")?;
        tracing::info!(">>> Encrypting token...");
        Ok(local::encrypt(sk, &claims, None, None)?)
    }

    fn from_encrypted_token(token: &str) -> Result<Self, super::error::AuthError> {
        let sk = super::symmetric_key_please().context("Can't has symmetric key")?;
        let rules = ClaimsValidationRules::new();
        let untrusted = UntrustedToken::<Local, V4>::try_from(token)?;
        let decoded = local::decrypt(sk, &untrusted, &rules, None, None)?;
        let claims = decoded
            .payload_claims()
            .ok_or(super::error::AuthError::ClaimsMissingError)?;
        let user = User::try_from(claims)?;
        Ok(user)
    }
}

impl TryInto<Claims> for User {
    type Error = super::error::AuthError;

    fn try_into(self) -> std::result::Result<Claims, Self::Error> {
        let duration = Duration::from_secs(60 * 60 * 24 * 7);
        let mut claims = Claims::new_expires_in(&duration)?;

        claims.subject(&self.email)?;
        claims.add_additional("fname", self.fname)?;
        claims.add_additional("lname", self.lname)?;
        if let Some(prv) = self.privileges {
            claims.add_additional("privileges", prv)?;
        }

        Ok(claims)
    }
}

impl TryFrom<&Claims> for User {
    type Error = super::error::AuthError;

    fn try_from(value: &Claims) -> std::result::Result<Self, Self::Error> {
        // FIXME: Better error
        // let user_json = value.get_claim("sub").context("What, no user?")?;
        let user_json = value.to_string().context("What, no seralize?")?;
        println!("{user_json}");

        let user = serde_json::from_str(&user_json)?;
        Ok(user)
    }
}

async fn user_with_privileges(pool: &Pool<Sqlite>, ticket: &Ulid) -> Result<User> {
    let code = ticket.to_string();
    let user_rows = sqlx::query!(
        r#"select mc.email, fname, lname, can as "can?" 
        from membership_confirmation as mc
        join privileges as pv
        where uuid = ?
        and mc.email = pv.email
        and mc.year = pv.valid"#,
        code
    )
    .fetch_all(pool)
    .await?;

    let mut iterator = user_rows.into_iter();

    let mut user: User = iterator
        .next()
        .map(|m| User::new(m.email, m.fname, m.lname, m.can.map(|c| vec![c])))
        .ok_or(Error::NotFound)?;

    for c in iterator.filter_map(|c| c.can) {
        if let Some(v) = user.privileges.as_mut() {
            v.push(c)
        }
    }

    Ok(user)
}

async fn _clean_tokens(ulid: Ulid, tx: &mut Transaction<'_, Sqlite>) -> Result<()> {
    let ulid_str = ulid.to_string();
    sqlx::query!(
        r#"
        delete from membership_confirmation
        where uuid = ? or timestamp < datetime('now', '-1 day')
        "#,
        ulid_str
    )
    .execute(tx.deref_mut())
    .await?;
    Ok(())
}

#[async_trait]
impl<S> FromRequestParts<S> for User
where
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header

        let cookies = Cookies::from_request_parts(parts, _state)
            .await
            .map_err(|_e| AuthError::ClaimsMissingError)?;

        let user = cookies
            .get(super::COOKIE_NAME)
            .ok_or(AuthError::ClaimsMissingError)
            .and_then(|c| (User::from_encrypted_token(c.value())))?;

        // Refresh expiration of cookie
        cookies.add(_bake_cookie(
            cookies.get(super::COOKIE_NAME).unwrap().value().to_owned(),
        ));

        Ok(user)
    }
}

fn week_long_expires() -> cookie::time::OffsetDateTime {
    let mut updated_expires = cookie::time::OffsetDateTime::now_utc();
    updated_expires += WEEK;
    updated_expires
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_token() {
        // Fixtures
        let usr_fx = User::new(
            "e@ma.il".to_string(),
            "Lars".to_string(),
            "Stefan".to_string(),
            Some(vec!["Cant::Do::Nothing".to_string()]),
        );
        let cmp = usr_fx.clone();

        let s = "k4.local.jhCHATQ3fmfna_QB1S3-8FEColfTWkaAnRa-1nFd3Rk";
        crate::CONFIG
            .set(crate::Metadata {
                token_key: s.to_owned(),
                ..Default::default()
            })
            .unwrap();

        // Run
        let claims = usr_fx.encrypt().unwrap();
        let des = User::from_encrypted_token(&claims).unwrap();

        let ser1 = serde_json::to_string_pretty(&des).unwrap();
        let ser2 = serde_json::to_string_pretty(&cmp).unwrap();
        assert_eq!(ser1, ser2);
    }
}
