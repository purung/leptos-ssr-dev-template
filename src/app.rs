use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

mod components;
use components::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/gongiversum.css"/>

        // sets the document title
        <Title text="Welcome to Leptor"/>
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
        <Nav />
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
        <main>
            <section class="bg-hoc-base-100 grid place-content-center min-h-screen w-full">
                <div class="flex max-w-2xl">
                    <div class="text-8xl font-bold text-accent">
                        <h2>
                            "Samtal som  " <span class="text-primary italic">"lindrar"</span> " och "
                            <span class="text-primary italic">"förändrar"</span>
                        </h2>
                    </div>
                    <div class="">"Bild"</div>
                </div>

            </section>
        </main>
    }
}

