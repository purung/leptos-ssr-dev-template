use std::sync::LazyLock;

use async_trait::async_trait;
use leptos::*;
use leptos_axum::redirect;
use serde::de::DeserializeOwned;
use serde::Serialize;
use ulid::Ulid;

use super::errors::EyeError;
use super::Contact;

static HOMEPAGE: LazyLock<String> =
    LazyLock::new(|| std::env::var("HOMEPAGE_URL").unwrap_or_default());

#[async_trait]
pub trait Communicate<Subject, Dialect>
where
    Subject: Serialize + DeserializeOwned,
{
    async fn power() -> Result<Dialect, EyeError>;
    async fn birth(&self) -> Result<(), EyeError>;
    async fn destroy(ulid: Ulid) -> Result<(), EyeError>;
    async fn all() -> Result<impl IntoIterator<Item = Subject>, EyeError>;
}

#[server]
pub async fn add_contact_request(
    name: String,
    tel: String,
    special: String,
) -> Result<(), ServerFnError> {
    Contact::new(name, tel, special).birth().await?;
    redirect(&HOMEPAGE);
    Ok(())
}

#[server]
pub async fn delete_contact_request(
    ulid: Ulid
) -> Result<(), ServerFnError> {
    Contact::destroy(ulid).await?;
    Ok(())
}

#[server]
pub async fn all_contact_requests() -> Result<Vec<Contact>, ServerFnError> {
    Ok(Contact::all().await?)
}
