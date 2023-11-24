use leptos::*;
use leptos_icons::*;
use leptos_query::{QueryResult, use_query, QueryOptions};
use leptos_router::A;
use leptos_animated_for::AnimatedFor;

use crate::app::please::{all_contact_requests, lol};

use super::Contact;

// #[component]
// pub fn Login() -> impl IntoView {
//     view! {}
// }

// #[component]
// pub fn ContactCard(card: Contact) -> impl IntoView {
//     view! {
//         <div class="card w-96 bg-primary text-primary-content">
//           <div class="card-body">
//             <h2 class="card-title">{ card.name }</h2>
//             <p>{ card.tel }</p>
//             <p>{ card.special }</p>
//             <div class="card-actions justify-end">
//               <button class="btn">Radera</button>
//             </div>
//           </div>
//         </div>
//     }
// }


#[component]
pub fn CardCollection() -> impl IntoView {
    let QueryResult { data, refetch, .. } = use_query(|| (), 
        |_| async move {
        lol().await.unwrap_or_default()
        }, 
        QueryOptions::default()
    );
    let cards = Signal::derive(move || data().unwrap_or_default());
    view! {
        <div class="place-self-start m-8">

             <AnimatedFor
                each=cards
                key=|card| card.stamp.clone()
                children=move |card| view! { <p>"Oj"</p> }
                enter_from_class="opacity-0"
                enter_class="duration-800"
                move_class="duration-1200"
                leave_class="opacity-0 duration-500"
                appear=true
            />
        </div>
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
