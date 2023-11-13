#![allow(non_snake_case)]

pub mod components;
pub mod pages;

use dioxus::prelude::*;
use log::LevelFilter;

fn main() {
     dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    dioxus_web::launch(app);
}

#[derive(Props, PartialEq)]
struct ShittyTextProps<'a> {
    text: &'a str,
}

#[derive(Props)]
pub struct FancyButtonProps<'a> {
    on_click: EventHandler<'a, MouseEvent>,
}

pub struct DarkMode(bool);

fn app(cx: Scope) -> Element {

    use_shared_state_provider(cx, || components::ipentry::EnteredIp("".to_string()));
    use_shared_state_provider(cx, || DarkMode(true));

    let dark_mode = use_shared_state::<DarkMode>(cx).unwrap();

    let body_class = if dark_mode.read().0 {
        "dark"
    } else {
        ""
    };

    cx.render(rsx! {
        Styling {},
        body {
            class: "{body_class}",
            components::navbar::NavBar {},
            pages::home::Home {}
        }
    })
}

pub fn Styling(cx: Scope) -> Element {
    cx.render(rsx! {
        style {
            include_str!("./assets/style.css")
         },
        style {
            include_str!("./assets/bulma/bulma-extensions.min.css")
        },
        script {
            include_str!("./assets/bulma/bulma-extensions.min.js")
        }
    })
}