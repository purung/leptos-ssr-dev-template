use leptos::*;
use leptos_icons::*;
use leptos_router::A;

// #[component]
// pub fn Login() -> impl IntoView {
//     view! {}
// }

// #[component]
// pub fn Contactcard() -> impl IntoView {
//     view! {}
// }

// #[component]
// pub fn Cardcollection() -> impl IntoView {
//     view! {}
// }

#[component]
pub fn Nav() -> impl IntoView {
    view! {
        <nav class="navbar bg-primary rounded-b-lg">
            <div class="navbar-start shrink">

                <span class="place-self-center ml-4 border-2 rounded-full border-base-100 ">
                    <Icon icon=Icon::from(IoIcon::IoTrash) class="w-10 h-10 p-1 text-base-100"/>
                </span>
            </div>
            <ul class="flex navbar-center w-1/2 max-w-screen-lg gap-12 justify-end">
                <li>
                    <A href="/vision">Vision</A>
                </li>
                <div class="dropdown">
                    <label tabindex="0" class="m-1 btn btn-ghost bg-inherit">
                        Services
                    </label>
                    <ul
                        tabindex="0"
                        class="menu dropdown-content z-[1] bg-base-100 shadow rounded-box w-52"
                    >
                        <li>
                            <A href="/services">"Food waste recycling"</A>
                        </li>
                        <li>
                            <A href="/services">"Trinket manangement"</A>
                        </li>
                        <li>
                            <A href="/services">"Scrap relocation"</A>
                        </li>
                    </ul>
                </div>
                <li>
                    <A href="/contact">Contact</A>
                </li>
            </ul>
            <div class="navbar-end">Icon</div>
        </nav>
    }
}
