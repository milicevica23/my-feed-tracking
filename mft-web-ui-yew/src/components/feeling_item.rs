use gloo::timers::callback::Timeout;
use yew::prelude::*;
use yewdux::prelude::use_store;

#[derive(Debug, PartialEq, Properties)]
pub struct Props {
    pub id: i64,
    pub name: String,
}

#[function_component(FeelingItem)]
pub fn feeling_item(props: &Props) -> Html {
    let Props { id, name } = props;

    use_effect_with_deps(move || {});

    html! {
    <div key={name}> class="">
        {name}
    </div>
    }
}
