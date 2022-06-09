use stylist::{css, style, yew::styled_component, Style};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub title: String,
    pub color: Color,
    pub on_load: Callback<String>,
}

#[derive(PartialEq)]
pub enum Color {
    Normal,
    Ok,
    Error,
}

impl Color {
    pub fn to_string(&self) -> String {
        match self {
            Color::Normal => "normal".to_owned(),
            Color::Ok => "ok".to_owned(),
            Color::Error => "error".to_owned(),
        }
    }
}

#[styled_component(MainTitle)]
// #[function_component(MainTitle)]
pub fn main_title(props: &Props) -> Html {
    let stylesheet = style!(
        r#"
        .normal{
            color: white;
        }

        .ok{
            color: green;
        }

        .error{
            color: red;
        }
    
        "#
    )
    .unwrap();

    props.on_load.emit("bim bada bomm".to_owned());
    html! {
        <div class={stylesheet}>
            <h1 class={props.color.to_string()}>{ &props.title}</h1>
        </div>
    }
}
