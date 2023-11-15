use leptos::*;

#[component]
pub fn Nav() -> impl IntoView {
    view! {
        <nav class="navbar">
            <div class="navbar-start"><span class="place-self-center">Logo</span></div>
            <div class="navbar-center">
                <ul class="flex grow max-w-screen-lg gap-8 justify-end">
                    <li>
                        <a href="/services">Services</a>
                    </li>
                    <li>
                        <a href="/vision">Vision</a>
                    </li>
                    <li>
                        <a href="/contact">Contact</a>
                    </li>
                </ul>
            </div>
        <div class="navbar-end">Icon</div>

        </nav>
    }
}

