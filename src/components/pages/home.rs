use crate::router::Router;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div>
            <h1>{"Home"}</h1>
            <div>
            <Link<Router> to={Router::Hello}>{"Click here to go hello"}</Link<Router>>
            </div>
        </div>
    }
}
