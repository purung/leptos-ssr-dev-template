use leptos::*;
// use leptos_icons::*;

use leptos_animated_for::AnimatedFor;
use leptos_router::ActionForm;


use crate::app::all_contact_requests;
use crate::app::please::*;

use super::Contact;

// #[component]
// pub fn Login() -> impl IntoView {
//     view! {}
// }

#[component]
pub fn ContactCard(card: Contact, reversion: WriteSignal<u32>) -> impl IntoView {
    let dispose = create_server_action::<DeleteContactRequest>();
    view! {
        <div class="card w-96 bg-accent text-base-100 rounded-lg shadow-lg self-stretch">
          <div class="card-body">
            <h2 class="card-title">{ card.name }</h2>
            <p>{ card.tel }</p>
            <p>{ card.special }</p>
            <div class="card-actions justify-end">
        <ActionForm action=dispose>
            <input type="hidden" name="ulid" value=move || card.stamp.to_string() />
            <button type="submit" class="btn" on:click=move |_| reversion.update(|v| *v += 1) >Radera</button>
        </ActionForm>
            </div>
          </div>
        </div>
    }
}

#[component]
pub fn CardCollection() -> impl IntoView {
    let (version, reversion) = RwSignal::new(0).split();
    let r = create_resource(
        move || version.get(),
        |_| async move { all_contact_requests().await.unwrap_or_default() },
    );
    let cards = Signal::derive(move || r.get().unwrap_or_default());
    view! {
        <Transition fallback=move || view! { <div class="place-self-center loading loading-dots text-base-100"></div> }>
            <ErrorBoundary fallback=move |_| view! {
                <div role="alert" class="alert alert-error place-self-center">
                  <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" /></svg>
                  <span>"Error! Task failed successfully."</span>
                </div>
             } >
                <div class="m-8 place-self-start grid grid-cols-2 gap-4">
                     <AnimatedFor
                        each=cards
                        key=|card| card.stamp
                        children=move |card| view! { <ContactCard card reversion /> }
                        enter_from_class="opacity-0"
                        enter_class="duration-800"
                        move_class="duration-1200"
                        leave_class="opacity-0 duration-1000"
                        appear=true
                    />
                </div>
            </ErrorBoundary>
        </Transition>
    }
}

//  <AnimatedFor
//     each=cards
//     key=|card| card.stamp
//     children=|card| view! { <ContactCard card /> }
//     enter_from_class="opacity-0"
//     enter_class="duration-800"
//     move_class="duration-1200"
//     leave_class="opacity-0 duration-500"
//     appear=true
// />
