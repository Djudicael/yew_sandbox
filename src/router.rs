use crate::components::pages::hello::Hello;
use crate::components::pages::home::Home;
use yew::prelude::*;
use yew_router::prelude::*;
#[derive(Debug, Clone, PartialEq, Routable)]
pub enum Router {
    #[at("/")]
    Home,
    #[at("/hello")]
    Hello,
}

pub fn switch(route: &Router) -> Html {
    match route {
        Router::Home => html! {<Home/>},
        Router::Hello => html! {<Hello/>},
    }
}
