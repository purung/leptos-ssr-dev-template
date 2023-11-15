use leptos::*;

#[component]
pub fn Nav() -> impl IntoView {
    view! {
        <nav class="w-full bg-primary flex">
            <div class="shrink grid p-2"><span class="place-self-center">Logo</span></div>
            <div class="flex grow bg-secondary py-4 justify-center">
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

        </nav>
    }
}

