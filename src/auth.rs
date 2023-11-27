use std::{collections::HashMap, env, time::Duration};

use async_trait::async_trait;
use axum::{
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
    response::IntoResponse,
};
use http::StatusCode;
use leptos::{leptos_config::Env, use_context, LeptosOptions};
use once_cell::sync::Lazy;
use pasetors::{
    claims::{Claims, ClaimsValidationRules},
    keys::SymmetricKey,
    local,
    token::UntrustedToken,
    version4::V4,
    Local,
};

use strum::IntoEnumIterator;
use thiserror::Error;
use tower_cookies::{cookie, Cookie, Cookies};

use crate::app::{User, MaybeUser};

static ENTRY_PHRASE: Lazy<HashMap<User, String>> = Lazy::new(|| {
    let mut book = HashMap::new();
    for usr in User::iter() {
        let pass = env::var(format!("{}_ENTRY_PASS", usr.as_ref()).to_uppercase())
            .expect("user pass to be set in the environment");
        book.insert(usr, pass);
    }
    book
});

const HOUR: cookie::time::Duration = cookie::time::Duration::seconds(60 * 60);
const COOKIE_NAME: &str = "passauth";
static SYMMETRIC_KEY: Lazy<SymmetricKey<V4>> = Lazy::new(|| {
    let key = env::var("COOKIE_SYMMETRIC_KEY").expect("cookie key to be set in the environment");
    SymmetricKey::<V4>::try_from(key.as_str()).expect("cookie key to be in the correct format")
});

async fn cookies() -> Result<Cookies, AuthError> {
    let ctx = use_context::<Cookies>().ok_or(AuthError::ConfigError)?;
    Ok(ctx)
}

async fn options() -> Result<LeptosOptions, AuthError> {
    let ctx = use_context::<LeptosOptions>().ok_or(AuthError::ConfigError)?;
    Ok(ctx)
}

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Fel användarnamn eller lösenord")]
    NotFound,
    #[error("Krypteringsproblem")]
    Encryption(#[from] pasetors::errors::Error),
    #[error("Saknar pass")]
    Missing,
    #[error("Fel inställningar")]
    ConfigError,
}

pub async fn confirm_login_for(user: String, password: String) -> Result<(), AuthError> {
    let cookies = cookies().await?;
    let options = options().await?;
    tracing::debug!(">>> Getting user");
    let user = User::try_from(user.as_str()).map_err(|_| AuthError::NotFound)?;
    user.password(password)?;
    let encrypted_user = user.encrypt()?;
    tracing::info!(">>> Adding cookie...");
    let cookie = _bake_cookie(encrypted_user, options);
    cookies.add(cookie);

    Ok(())
}

fn _bake_cookie<'c>(value: String, options: LeptosOptions) -> Cookie<'c> {
    let cookie = Cookie::build(COOKIE_NAME, value)
        .max_age(HOUR)
        .expires(hour_long_expires())
        .http_only(true)
        .path("/");

    if options.env == Env::PROD {
        return cookie
            .secure(true)
            .same_site(cookie::SameSite::Strict)
            .finish();
    } else {
        cookie.finish()
    }
}

impl User {
    fn encrypt(self) -> Result<String, AuthError> {
        tracing::info!(">>> Generating claims...");
        let claims: Claims = self.try_into()?;
        tracing::info!(">>> Encrypting token...");
        Ok(local::encrypt(&SYMMETRIC_KEY, &claims, None, None)?)
    }

    fn from_encrypted_token(token: &str) -> Result<Self, AuthError> {
        let rules = ClaimsValidationRules::new();
        let untrusted = UntrustedToken::<Local, V4>::try_from(token)?;
        let decoded = local::decrypt(&SYMMETRIC_KEY, &untrusted, &rules, None, None)?;
        let claims = decoded.payload_claims().ok_or(AuthError::Missing)?;
        let user = User::try_from(claims)?;
        Ok(user)
    }
    fn password(&self, password: String) -> Result<(), AuthError> {
        if let Some(correct) = ENTRY_PHRASE.get(self) {
            if &password == correct {
                return Ok(());
            } else {
                return Err(AuthError::NotFound);
            }
        } else {
            return Err(AuthError::NotFound);
        };
    }
}

impl TryInto<Claims> for User {
    type Error = AuthError;

    fn try_into(self) -> Result<Claims, Self::Error> {
        let duration = Duration::from_secs(60 * 60);
        let mut claims = Claims::new_expires_in(&duration)?;

        claims.subject(self.as_ref())?;

        Ok(claims)
    }
}

impl TryFrom<&Claims> for User {
    type Error = AuthError;

    fn try_from(value: &Claims) -> std::result::Result<Self, Self::Error> {
        let user = value
            .get_claim("sub")
            .and_then(|v| v.as_str())
            .ok_or(AuthError::Missing)?;
        Ok(Self::try_from(user).map_err(|_| AuthError::Missing)?)
    }
}

impl IntoResponse for AuthError {
    fn into_response(self) -> axum::response::Response {
        StatusCode::UNAUTHORIZED.into_response()
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for MaybeUser
where
    S: Send + Sync,
    LeptosOptions: FromRef<S>,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header

        let user = _get_user_or_fail_trying(parts, _state).await;
        // tracing::warn!(">>>>>>>>>>>>>> Fångade in användare: {:?}", user);
        match user {
            Ok(u) => Ok(MaybeUser::User(u)),
            Err(e) => match e {
                AuthError::NotFound | AuthError::ConfigError | AuthError::Missing => {
                    Ok(MaybeUser::None)
                }
                AuthError::Encryption(_) => Ok(MaybeUser::Rejected),
            },
        }
    }
}

async fn _get_user_or_fail_trying<S>(parts: &mut Parts, state: &S) -> Result<User, AuthError>
where
    S: Send + Sync,
    LeptosOptions: FromRef<S>,
{
    let cookies = Cookies::from_request_parts(parts, state)
        .await
        .map_err(|_e| AuthError::Missing)?;

    let user = cookies
        .get(COOKIE_NAME)
        .ok_or(AuthError::Missing)
        .and_then(|c| (User::from_encrypted_token(c.value())))?;

    let options = LeptosOptions::from_ref(state);

    // Refresh expiration of cookie
    cookies.add(_bake_cookie(
        cookies.get(COOKIE_NAME).unwrap().value().to_owned(),
        options,
    ));
    Ok(user)
}

fn hour_long_expires() -> cookie::time::OffsetDateTime {
    let mut updated_expires = cookie::time::OffsetDateTime::now_utc();
    updated_expires += HOUR;
    updated_expires
}
