use gloo::console::log;
use serde::{Deserialize, Serialize};
use stylist::{css, style, yew::styled_component, Style};
use yew::prelude::*;

mod components;
use components::atoms::main_title::{Color, MainTitle};
use components::molecules::custom_form::CustomForm;

use crate::components::molecules::custom_form::Data;

const STYLE_FILE: &str = include_str!("main.css");
#[derive(Serialize, Deserialize)]
struct MyObject {
    username: String,
    favorite_language: String,
}

// #[function_component(App)]
#[styled_component(App)]
pub fn app() -> Html {
    let stylesheet = Style::new(STYLE_FILE).unwrap();
    let name = "brooks";
    let my_object = MyObject {
        username: name.to_owned(),
        favorite_language: "rust".to_owned(),
    };
    log!("my name is ", name);
    log!(serde_json::to_string_pretty(&my_object).unwrap());
    let class = "my_titles";
    // let message: Option<&str> = Some("I am a message");
    let message: Option<&str> = None;

    let tasks = vec!["record video", "grocery shopping", "pet"];

    let main_title_load = Callback::from(|message: String| log!(message));
    let custom_form_submit = Callback::from(|data: Data| {
        log!("username is ", data.username);
        log!("favorite language is ", data.favorite_language);
    });

    html! {
        <>
            <div class={stylesheet}>
                <MainTitle title="hi there" color={Color::Error} on_load={main_title_load}/>
                <CustomForm onsubmit={custom_form_submit}/>
                <h1>{"goldora"}</h1>
                if class=="my_title"{
                    <p> {"hola amiga "}</p>
                } else {
                    <p> {"i am not a title "}</p>

                }

                if let Some(message)= message {
                    <p> {message}</p>

                }else {
                    <p>{" No message to see today"}</p>
                }

                <ul>
                    {list_to_html(tasks)}
                </ul>
            </div>
        </>
    }
}

fn list_to_html(list: Vec<&str>) -> Vec<Html> {
    list.iter()
        .map(|task| html! {<li class={css!("color: red; font-size: 75px;")}>{task}</li>})
        .collect()
}
