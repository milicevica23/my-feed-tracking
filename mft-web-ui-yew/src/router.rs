use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::{
    feeling_page::FeelingPage, home_page::HomePage, login_page::LoginPage,
    profile_page::ProfilePage, register_page::RegisterPage,
};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    HomePage,
    #[at("/register")]
    RegisterPage,
    #[at("/login")]
    LoginPage,
    #[at("/profile")]
    ProfilePage,
    #[at("/feeling")]
    FeelingPage,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::HomePage => html! {<HomePage/> },
        Route::RegisterPage => html! {<RegisterPage/> },
        Route::LoginPage => html! {<LoginPage/> },
        Route::ProfilePage => html! {<ProfilePage/> },
        Route::FeelingPage => html! {<FeelingPage/>},
    }
}
