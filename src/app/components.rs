use leptos::*;
use leptos_icons::*;

// use leptos_animated_for::AnimatedFor;
use leptos_router::ActionForm;

use crate::app::all_contact_requests;
use crate::app::please::*;

use super::Contact;

#[component]
pub fn Login() -> impl IntoView {
    let gogogo = create_server_action::<LogMeIn>();
    let error = Signal::derive(move || match gogogo.value().get() {
        Some(r) => match r {
            Ok(_) => None,
            Err(e) => e.to_string().split_once(": ").and_then(|s| Some(s.1.to_owned())),
        },
        None => None,
    });
    view! {
        <main class="bg-primary min-h-[100svh] grid">
            <div class="place-self-center px-8 py-12 w-[90vw] max-w-sm bg-accent rounded-lg shadow-lg">
                <ActionForm action=gogogo class="flex flex-col gap-8">
                    <h4 class="text-lg text-base-100 text-center font-bold">Logga in</h4>
                    <p class="text-center text-base-100">{ move || error.get() }</p>
                    <div class="form-control">
                        <label class="label text-base-100">Namn</label>
                        <input
                            name="user"
                            inputmode="text"
                            required
                            class="input input-bordered input-base-100 w-full"
                        />
                    </div>
                    <div class="form-control">
                        <label class="label text-base-100">Lösenord</label>
                        <input
                            name="password"
                            type="password"
                            required
                            class="input input-bordered input-base-100 w-full"
                        />
                    </div>
                    <div class="form-control grow">
                        <button
                            class="btn btn-secondary hover:btn-outline text-accent hover:btn-secondary"
                            type="submit"
                            id="skickaKnapp"
                            >
                            <Show when=move || gogogo.pending().get() fallback=move || view! {  <span id="skicka">Logga in</span> }>
                                <span id="laddar" class="loading loading-dots loading-sm" > </span>
                            </Show>
                        </button>
                    </div>
                </ActionForm>
            </div>
        </main>
    }
}

#[component]
pub fn ContactCard(card: Contact, reversion: Callback<()>) -> impl IntoView {
    let dispose = create_server_action::<DeleteContactRequest>();
    create_effect(move |_| {
        dispose.version().get();
        leptos::Callable::call(&reversion, ());
    });
    let tel_link = card.tel_link();
    let human_ts = card.human_timestamp();
    view! {
        <div class="card w-96 relative bg-accent text-base-100 rounded-lg shadow-lg self-stretch">
          <div class="card-body">
            <h2 class="card-title mt-6 mb-3">{ card.name }</h2>
            <p>{ card.tel }</p>
            <p class="text-xs absolute right-0 top-0 p-4 text-secondary">{ human_ts }</p>
            <p class="my-6">{ card.special }</p>
            <div class="card-actions mt-24 justify-between content-center">
        <a href=tel_link class="btn btn-ghost btn-md text-base-100/60 hover:text-base-100">
        <Icon icon=Icon::from(IoIcon::IoCall) class="w-full h-full" />
        </a>

        <ActionForm action=dispose >
            <input type="hidden" name="ulid" value=move || card.stamp.to_string() />

            <button type="submit" class="btn btn-md text-base-100/60 hover:text-base-100 btn-ghost">
        <Icon class="w-full h-full" icon=Icon::from(IoIcon::IoCheckmarkDoneCircleSharp) />
        </button>
        </ActionForm>
            </div>
          </div>
        </div>
    }
}

#[component]
pub fn CardCollection() -> impl IntoView {
    let r = create_resource(
        || (),
        |_| async move { all_contact_requests().await.unwrap_or_default() },
    );
    let cards = Signal::derive(move || r.get().unwrap_or_default());
    let reversion = Callback::new(move |()| r.refetch());
    view! {
        <Transition fallback=move || view! { <div class="place-self-center loading loading-dots text-base-100"></div> }>
            <ErrorBoundary fallback=move |_| view! {
                <div role="alert" class="alert alert-error place-self-center">
                  <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" /></svg>
                  <span>"Error! Task failed successfully."</span>
                </div>
             } >
                <h1 class="text-3xl text-center text-accent m-12 font-bold">Uppringningslista</h1>
                <div class="m-8 grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4 max-w-screen-xl mx-auto">
                     <For
                        each=cards
                        key=|card| card.stamp
                        children=move |card| view! { <ContactCard card reversion /> }
                    />
                </div>
            </ErrorBoundary>
        </Transition>
    }
}
