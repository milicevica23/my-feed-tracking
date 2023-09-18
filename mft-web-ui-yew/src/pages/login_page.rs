use std::ops::Deref;

use crate::api::user_api::api_login_user;
use crate::components::{form_input::FormInput, loading_button::LoadingButton};
use crate::router::{self, Route};
use crate::store::{set_auth_user, set_page_loading, set_show_alert, Store};

use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use web_sys::{console, HtmlInputElement};
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
struct LoginUserSchema {
    email: String,
    password: String,
}

fn get_input_callback(
    name: &'static str,
    cloned_form: UseStateHandle<LoginUserSchema>,
) -> Callback<String> {
    Callback::from(move |value| {
        let mut data = cloned_form.deref().clone();
        match name {
            "email" => data.email = value,
            "password" => data.password = value,
            _ => (),
        }
        cloned_form.set(data);
    })
}

#[function_component(LoginPage)]
pub fn login_page() -> Html {
    let (store, dispatch) = use_store::<Store>();
    let form = use_state(|| LoginUserSchema::default());
    let navigator = use_navigator().unwrap();

    let email_input_ref = NodeRef::default();
    let password_input_ref = NodeRef::default();

    let handle_email_input = get_input_callback("email", form.clone());
    let handle_password_input = get_input_callback("password", form.clone());

    let on_submit = {
        let store_dispatch = dispatch.clone();
        let cloned_navigator = navigator.clone();

        let cloned_email_input_ref = email_input_ref.clone();
        let cloned_password_input_ref = password_input_ref.clone();

        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();

            let dispatch = store_dispatch.clone();
            let navigator = cloned_navigator.clone();

            let email_input_ref = cloned_email_input_ref.clone();
            let password_input_ref = cloned_password_input_ref.clone();

            spawn_local(async move {
                set_page_loading(true, dispatch.clone());

                let email_input = email_input_ref.cast::<HtmlInputElement>().unwrap();
                let password_input = password_input_ref.cast::<HtmlInputElement>().unwrap();

                let user = api_login_user(
                    email_input.value().as_str().to_owned(),
                    password_input.value().as_str().to_owned(),
                )
                .await;

                console::log_1(&format!("{:#?}", user).into());
                set_auth_user(Some(user.clone().unwrap()), dispatch.clone());
                match user {
                    Ok(_) => {
                        //set_page_loading(false, dispatch);
                        navigator.push(&router::Route::ProfilePage);
                    }
                    Err(e) => {
                        //set_page_loading(false, dispatch.clone());
                        set_show_alert(e.to_string(), dispatch);
                    }
                };
            });
        })
    };

    html! {
    <section class="bg-ct-blue-600 min-h-screen grid place-items-center">
      <div class="w-full">
        <h1 class="text-4xl xl:text-6xl text-center font-[600] text-ct-yellow-600 mb-4">
          {"Welcome Back"}
        </h1>
        <h2 class="text-lg text-center mb-4 text-ct-dark-200">
          {"Login to have access"}
        </h2>
          <form
            onsubmit={on_submit}
            class="max-w-md w-full mx-auto overflow-hidden shadow-lg bg-ct-dark-200 rounded-2xl p-8 space-y-5"
          >
            <FormInput label="Name" name="email" input_ref={email_input_ref} handle_onchange={handle_email_input} />
            <FormInput label="Password" name="password" input_type="password" input_ref={password_input_ref} handle_onchange={handle_password_input} />

            <div class="text-right">
              <a href="#">
                {"Forgot Password?"}
              </a>
            </div>
            <LoadingButton
              loading={store.page_loading}
              text_color={"text-ct-blue-600"}
            >
              {"Login"}
            </LoadingButton>
            <span class="block">
              {"Need an account?"} {" "}
              <Link<Route> to={Route::RegisterPage} classes="text-ct-blue-600">{ "Sign Up Here" }</Link<Route>>
            </span>
          </form>
      </div>
    </section>
    }
}
