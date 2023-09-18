use std::ops::Deref;

use crate::api::user_api::api_register_user;
use crate::components::{form_input::FormInput, loading_button::LoadingButton};
use crate::router::{self, Route};
use crate::store::Store;

use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]

struct RegisterUserSchema {
    name: String,
    password: String,
}

fn get_input_callback(
    name: &'static str,
    cloned_form: UseStateHandle<RegisterUserSchema>,
) -> Callback<String> {
    Callback::from(move |value| {
        let mut data = cloned_form.deref().clone();
        match name {
            "name" => data.name = value,
            "password" => data.password = value,
            _ => (),
        }
        cloned_form.set(data);
    })
}

#[function_component(RegisterPage)]
pub fn register_page() -> Html {
    let (store, dispatch) = use_store::<Store>();
    let form = use_state(|| RegisterUserSchema::default());
    let navigator = use_navigator().unwrap();

    let name_input_ref = NodeRef::default();
    let password_input_ref = NodeRef::default();

    let handle_name_input = get_input_callback("name", form.clone());
    let handle_password_input = get_input_callback("password", form.clone());

    let on_submit = {
        let cloned_navigator = navigator.clone();

        let cloned_name_input_ref = name_input_ref.clone();
        let cloned_password_input_ref = password_input_ref.clone();

        Callback::from(move |event: SubmitEvent| {
            let navigator = cloned_navigator.clone();

            let name_input_ref = cloned_name_input_ref.clone();
            let password_input_ref = cloned_password_input_ref.clone();

            event.prevent_default();
            spawn_local(async move {
                let name_input = name_input_ref.cast::<HtmlInputElement>().unwrap();

                let password_input = password_input_ref.cast::<HtmlInputElement>().unwrap();

                let res = api_register_user(
                    name_input.value().as_str().to_owned(),
                    password_input.value().as_str().to_owned(),
                )
                .await;
                match res {
                    Ok(_) => {
                        navigator.push(&router::Route::LoginPage);
                    }
                    Err(e) => {
                        navigator.push(&router::Route::LoginPage);
                    }
                };
            })
        })
    };

    html! {
    <section class="py-8 bg-ct-blue-600 min-h-screen grid place-items-center">
      <div class="w-full">
        <h1 class="text-4xl xl:text-6xl text-center font-[600] text-ct-yellow-600 mb-4">
         {" Welcome to CodevoWeb!"}
        </h1>
        <h2 class="text-lg text-center mb-4 text-ct-dark-200">
          {"Sign Up To Get Started!"}
        </h2>
          <form
            onsubmit={on_submit}
            class="max-w-md w-full mx-auto overflow-hidden shadow-lg bg-ct-dark-200 rounded-2xl p-8 space-y-5"
          >
            <FormInput label="Name" name="name" input_ref={name_input_ref} handle_onchange={handle_name_input} />
            <FormInput label="Password" name="password" input_type="password" input_ref={password_input_ref} handle_onchange={handle_password_input}  />

            <span class="block">
              {"Already have an account?"} {" "}
            <Link<Route> to={Route::LoginPage} classes="text-ct-blue-600">{"Login Here"}</Link<Route>>
            </span>
            <LoadingButton
              loading={store.page_loading}
              text_color={"text-ct-blue-600"}
            >
             {" Sign Up"}
            </LoadingButton>
          </form>
      </div>
    </section>
    }
}
