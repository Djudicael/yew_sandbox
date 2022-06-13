use std::ops::Deref;

use yew::prelude::*;

use crate::components::atoms::custom_button::CustomButton;
use crate::components::atoms::text_input::TextInput;
use crate::User;

#[derive(Default, Clone)]
pub struct Data {
    pub username: String,
    pub favorite_language: String,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub onsubmit: Callback<Data>,
}

#[function_component(CustomForm)]
pub fn custom_form(props: &Props) -> Html {
    let state = use_state(|| Data::default());
    let user_context = use_context::<User>();

    let cloned_state = state.clone();
    let username_change = Callback::from(move |username| {
        cloned_state.set(Data {
            username,
            ..cloned_state.deref().clone()
        })
    });
    let cloned_state = state.clone();
    let language_change = Callback::from(move |language| {
        cloned_state.set(Data {
            favorite_language: language,
            ..cloned_state.deref().clone()
        })
    });

    let form_onsubmit = props.onsubmit.clone();
    let cloned_state = state.clone();

    let onsubmit = Callback::from(move |event: FocusEvent| {
        event.prevent_default();
        let data = cloned_state.deref().clone();
        form_onsubmit.emit(data);
    });

    html! {
        <form onsubmit={onsubmit}>
            <TextInput name="username" handle_onchange={username_change}/>
            <TextInput name="favorite language " handle_onchange={language_change}/>
            <CustomButton label="Submit" />
            <p>{"Username "}{user_context.clone().unwrap_or_default().username}</p>
            <p>{"favorite language "}{user_context.unwrap_or_default().favorite_language}</p>

        </form>
    }
}
