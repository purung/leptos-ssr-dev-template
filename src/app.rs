use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/gongiversum.css"/>

        // sets the document title
        <Title text="Welcome to Leptor"/>

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
    // Creates a reactive value to update the button
    // let (count, set_count) = create_signal(0);
    // let on_click = move |_| set_count.update(|count| *count += 3);
    let (_, y) = leptos_use::use_window_scroll();

    view! {
        <section class="flex flex-col justify-end min-h-screen">
            x: {move || format!("{:.1}", y())}
            <div
                class="w-full bg-amber-200 h-[100px]"
                style:transform=move || format!("translateY(-{}px)", y() * 0.7)
            >
                A
            </div>
            <div
                class="w-full bg-amber-400 h-[100px]"
                style:transform=move || format!("translateY(-{}px)", y())
            >
                A
            </div>
            <div
                class="w-full bg-amber-600 h-[100px]"
                style:transform=move || format!("translateY(-{}px)", y() * 1.3)
            >
                A
            </div>
        </section>
        <section class="h-[500vh]">D</section>
    }
}

