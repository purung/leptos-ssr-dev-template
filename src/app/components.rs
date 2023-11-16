use leptos::*;
use leptos_router::A;

#[component]
pub fn Nav() -> impl IntoView {
    view! {
        <nav class="navbar bg-primary rounded-b-lg">
            <div class="navbar-start shrink"><span class="place-self-center">Logo</span></div>
                <ul class="flex navbar-center w-1/2 max-w-screen-lg gap-12 justify-end">
                    <li>
                        <A href="/services">Services</A>
                    </li>
                    <li>
                        <A href="/vision">Vision</A>
                    </li>
                    <li>
                        <A href="/contact">Contact</A>
                    </li>
                </ul>
        <div class="navbar-end">Icon</div>

        </nav>
    }
}

