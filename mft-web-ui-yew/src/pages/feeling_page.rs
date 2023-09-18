use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

use crate::components::header::Header;

#[function_component(FeelingPage)]
pub fn feeling_page() -> Html {
    let feeling_items = vec!["aleks".to_string(), "burcu".to_string()];

    html! {
    <>
        <Header />
        <section class="bg-ct-blue-600 min-h-screen pt-20">
        {
            feeling_items.into_iter().map(move |name| {
                feeling_item(props!(Props {id: 1, name}));
            } ).collect::<Html>()

        }
        </section>
    </>
    }
}
