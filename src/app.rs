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
        <Stylesheet id="leptos" href="/pkg/airborne_trash_patrol.css"/>

        // sets the document title
        <Title text="Airborne Trash Platoon"/>
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
            <Nav/>
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
                <div class="max-w-[80vw] grid gap-8 grid-cols-1 md:grid-cols-2" >
                    <div class="grid place-content-center">
                        <h2 class="text-6xl font-bold text-accent m-8">
                            "Sudden cleanliness "
                            <span class="text-primary italic">"from above"</span>
                        </h2>
                        <p class="text-xl m-8">
                            "On the fly precision waste removal by the Airborne Trash Platoon"
                        </p>
                        <div class="grid grid-cols-smol gap-4 m-8" style="--min: 15rem;">
                            <A href="/offers" class="btn btn-outline btn-primary">
                                "See current offers "
                            </A>
                            <A href="/order" class="btn btn-accent text-base-100">
                                "Request pickup"
                            </A>
                        </div>
                    </div>
                    <div class="order-first md:order-last">
                        <img src="hero.png"/>
                    </div>
                </div>
            </section>
        </main>
    }
}

