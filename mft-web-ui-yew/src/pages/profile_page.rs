use crate::api::types::User;
use crate::{
    //api::user_api::{api_refresh_access_token, api_user_info},
    components::header::Header,
    router,
    store::{set_auth_user, set_page_loading, set_show_alert, Store},
};
use yew::prelude::*;
use yew_router::prelude::use_navigator;
use yewdux::prelude::*;

#[function_component(ProfilePage)]
pub fn profile_page() -> Html {
    let (store, dispatch) = use_store::<Store>();
    let user = store.auth_user.clone();
    let navigator = use_navigator().unwrap();
    /*let user = Some(User {
        id: 1,
        name: "aleks".to_string(),
        access_token: "".to_string(),
    });*/
    html! {
    <>
      <Header />
      <section class="bg-ct-blue-600 min-h-screen pt-20">
        <div class="max-w-4xl mx-auto bg-ct-dark-100 rounded-md h-[20rem] flex justify-center items-center">
          <div>
            <p class="text-5xl font-semibold">{"Profile Page"}</p>
            if let Some(user) = user {
              <div class="mt-8">
                <p class="mb-4">{format!("ID: {}", user.id)}</p>
                <p class="mb-4">{format!("Name: {}", user.name)}</p>
              </div>
            }else {
              <p class="mb-4">{"Loading..."}</p>
            }
          </div>
        </div>
      </section>
    </>
    }
}
