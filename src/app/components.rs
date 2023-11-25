use leptos::*;
// use leptos_icons::*;
use leptos_router::A;
use leptos_animated_for::AnimatedFor;

use crate::app::please::lol;
// use crate::app::please::all_contact_requests;

use super::Contact;

// #[component]
// pub fn Login() -> impl IntoView {
//     view! {}
// }

#[component]
pub fn ContactCard(card: Contact) -> impl IntoView {
    view! {
        <div class="card w-96">
          <div class="card-body bg-accent">
            <h2 class="card-title">{ card.name }</h2>
            <p>{ card.tel }</p>
            <p>{ card.special }</p>
            <div class="card-actions justify-end">
              <button class="btn">Radera</button>
            </div>
          </div>
        </div>
    }
}


#[component]
pub fn CardCollection() -> impl IntoView {
    // let QueryResult { data, refetch, .. } = use_query(|| (), 
    //     |_| async move {
    //     lol().await.unwrap_or_default()
    //     }, 
    //     QueryOptions::default()
    // );
    let r = create_resource(||(), |_| async move {
        lol().await.unwrap_or_default()
    });
    let cards = Signal::derive(move || r.get().unwrap_or_default());
    view! {
        <Transition fallback=move || view! { <div class="place-self-center loading loading-dots"></div> }>
            <ErrorBoundary fallback=move |_| view! {
                <div role="alert" class="alert alert-error place-self-center">
                  <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" /></svg>
                  <span>"Error! Task failed successfully."</span>
                </div>
             } >

                <div class="m-8 place-self-start">
                     <AnimatedFor
                        each=cards
                        key=|card| card.stamp.clone()
                        children=move |card| view! { <ContactCard card /> }
                        enter_from_class="opacity-0"
                        enter_class="duration-800"
                        move_class="duration-1200"
                        leave_class="opacity-0 duration-500"
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
