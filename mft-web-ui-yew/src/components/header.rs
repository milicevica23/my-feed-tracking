use crate::{
    router::{self, Route},
    store::{set_auth_user, Store},
};
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::{dispatch, prelude::*};

#[function_component(Header)]
pub fn header_component() -> Html {
    let (store, dispatch) = use_store::<Store>();
    let user = store.auth_user.clone();
    let navigator = use_navigator().unwrap();
    let delete_user = {
        Callback::from(move |event: MouseEvent| {
            set_auth_user(None, dispatch.clone());
            navigator.push(&router::Route::LoginPage);
        })
    };
    html! {
        <header class="bg-white h-20">
        <nav class="h-full flex justify-between container items-center">
          <div>
            <Link<Route> to={Route::HomePage} classes="text-ct-dark-600">{"MFT"}</Link<Route>>
          </div>
          <ul class="flex items-center gap-4">
            <li>
              <Link<Route> to={Route::HomePage} classes="text-ct-dark-600">{"Home"}</Link<Route>>
            </li>
            <li>
              <Link<Route> to={Route::FeelingPage} classes="text-ct-dark-600">{"Feeling Page"}</Link<Route>>
            </li>
            if user.is_some() {
               <>
                <li>
                  <Link<Route> to={Route::ProfilePage} classes="text-ct-dark-600">{"Profile"}</Link<Route>>
                </li>
                <li
                  class="cursor-pointer"
                >
                  {"Create Post"}
                </li>
                <li onclick={delete_user} class="cursor-pointer">
                  {"Logout"}
                </li>
              </>

            } else {
              <>
                <li>
                  <Link<Route> to={Route::RegisterPage} classes="text-ct-dark-600">{"SignUp"}</Link<Route>>
                </li>
                <li>
                  <Link<Route> to={Route::LoginPage} classes="text-ct-dark-600">{"Login"}</Link<Route>>
                </li>
              </>
            }
          </ul>
        </nav>
      </header>
    }
}
