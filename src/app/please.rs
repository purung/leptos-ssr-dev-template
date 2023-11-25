// #[cfg(feature = "ssr")]
// use std::sync::LazyLock;

use leptos::*;
// #[cfg(feature = "ssr")]
// use leptos_axum::redirect;

use serde::de::DeserializeOwned;
use serde::Serialize;
use ulid::Ulid;

#[cfg(feature = "ssr")]
use async_trait::async_trait;
#[cfg(feature = "ssr")]
use super::errors::EyeError;

use super::Contact;

// #[cfg(feature = "ssr")]
// static HOMEPAGE: LazyLock<String> =
//     LazyLock::new(|| std::env::var("HOMEPAGE_URL").unwrap_or_default());

#[cfg(feature = "ssr")]
#[async_trait]
pub trait Communicate<Subject, Dialect>
where
    Subject: Serialize + DeserializeOwned,
{
    async fn power() -> Result<Dialect, EyeError>;
    async fn birth(&self) -> Result<(), EyeError>;
    async fn destroy(ulid: Ulid) -> Result<(), EyeError>;
    async fn all() -> Result<Vec<Subject>, EyeError>;
}

// #[server]
// pub async fn add_contact_request(
//     name: String,
//     tel: String,
//     special: String,
// ) -> Result<(), ServerFnError> {
//     Contact::new(name, tel, special).birth().await?;
//     redirect(&HOMEPAGE);
//     Ok(())
// }

#[server]
pub async fn delete_contact_request(
    ulid: Ulid
) -> Result<(), ServerFnError> {
    Contact::destroy(ulid).await.unwrap();
    Ok(())
}

#[server]
pub async fn all_contact_requests() -> Result<Vec<Contact>, ServerFnError> {
    Ok(Contact::all().await?)
}

#[server(Lol, "/api")]
pub async fn lol() -> Result<Vec<Contact>, ServerFnError> {
    Ok(vec![Contact::default()])
}
