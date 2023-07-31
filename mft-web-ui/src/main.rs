use gloo_net::http::Request;
use mft_domain::Feeling;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
struct FeelingListProps {
    feelings: Vec<Feeling>,
}

#[function_component(FeelingsList)]
fn feeling_list(FeelingListProps { feelings }: &FeelingListProps) -> Html {
    feelings
        .iter()
        .map(|feeling| {
            html! {
                <p key={feeling.id}>{format!("{}: {}", feeling.id, feeling.name)}</p>
            }
        })
        .collect()
}

#[function_component(App)]
fn app() -> Html {
    let feelings = vec![
        Feeling {
            id: 1,
            name: "flying".to_string(),
        },
        Feeling {
            id: 2,
            name: "finger swallow".to_string(),
        },
    ];
    let feelings = use_state(|| vec![]);
    {
        let feelings = feelings.clone();
        use_effect_with_deps(
            move |_| {
                let feelings = feelings.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_feelings: Vec<Feeling> =
                        Request::get("http://localhost:8080/feeling")
                            .send()
                            .await
                            .unwrap()
                            .json()
                            .await
                            .unwrap();
                    feelings.set(fetched_feelings);
                });
                || ()
            },
            (),
        );
    }
    html! {
        <>
            <h1>{ "Feelings: " }</h1>
            <FeelingsList feelings={(*feelings).clone()} />
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
