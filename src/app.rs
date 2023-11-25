use crate::error_template::{AppError, ErrorTemplate};
use chrono::{DateTime, Local};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use ulid::Ulid;

use serde::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
pub mod db;

pub mod components;
pub use components::*;
pub mod please;
pub use please::*;
pub mod errors;
pub use errors::*;

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct Contact {
    stamp: Ulid,
    name: String,
    tel: String,
    special: String,
    timestamp: DateTime<Local>,
}

impl Contact {
    fn new(name: String, tel: String, special: String) -> Self {
        Self {
            name,
            tel,
            special,
            timestamp: Local::now(),
            stamp: Ulid::new(),
        }
    }
}

impl Default for Contact {
    fn default() -> Self {
        let name = "Inigo Montoya".to_owned();
        let tel = "070 666 666".to_owned();
        let special = "You killed my father. Prepare to die.".to_owned();
        Self::new(name, tel, special)
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/birds-psy.css"/>

        // sets the document title
        <Title text="Ovanifrånvy"/>
        <Link rel="preconnect" href="https://fonts.googleapis.com"/>
        <Link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="crossorigin"/>
        <Link
            href="https://fonts.googleapis.com/css2?family=Noto+Sans:ital,wght@0,300;0,400;0,700;1,400;1,700&display=swap"
            rel="stylesheet"
        />
        // content for this welcome page

        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors/> }.into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    view! {
        <main class="bg-primary h-[100svh] grid">
            <CardCollection /> 
        </main>
    }
}
