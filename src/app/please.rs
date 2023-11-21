use leptos::*;
use serde::de::DeserializeOwned;
use serde::Serialize;
use ulid::Ulid;

use super::{Contact, errors::EyeError};

pub trait Communicate<Subject, Veichle> where Subject: Serialize + DeserializeOwned {
    const LABEL: &'static str;

    fn birth(self) -> Result<Subject, EyeError>;
    fn destroy(ulid: Ulid) -> Result<(), EyeError>;
    fn all() -> Result<impl IntoIterator, EyeError>;
    
}

#[server]
pub async fn add_contact_request(name: String, tel: String, special: String) -> Result<(), ServerFnError> {



    Ok(())
    }
