#![allow(non_snake_case)]

pub mod components;
pub mod pages;
pub mod utils;

use dioxus::prelude::*;
use dioxus_router::prelude::*;
use log::LevelFilter;
use utils::antivpn::KauriResponse;
use pages::home::Home;
use pages::ipresult::IpResult;


fn main() {
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    dioxus_web::launch(app);
}

pub struct SharedKauriResponse(Option<KauriResponse>);

pub struct DarkMode(bool);

#[derive(Routable, Clone)]
pub enum Route {

    #[route("/")]
    Home {},
    #[route("/ipresult/:ipToCheck")]
    IpResult {ipToCheck: String},
    #[end_layout]
    #[route("/:..route")]
	NotFound { route: Vec<String> }


}
fn app(cx: Scope) -> Element {

    use_shared_state_provider(cx, || components::ipentry::EnteredIp("".to_string()));
    use_shared_state_provider(cx, || DarkMode(true));
    use_shared_state_provider(cx, || SharedKauriResponse(None));


    cx.render(rsx! {
        Router::<Route> {}
    })
}

#[inline_props]
pub fn Formatted<'a>(cx: Scope<'a>, children: Element<'a>) -> Element {
    let dark_mode = use_shared_state::<DarkMode>(cx).unwrap();

    let body_class = if dark_mode.read().0 {
        "dark"
    } else {
        ""
    };

    render! {
        Styling {},
        body {
            class: "{body_class}",
            components::navbar::NavBar {},
            div {
                children
            }
        }
    }
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
#[inline_props]
fn NotFound(cx: Scope, route: Vec<String>) -> Element {
    render! {
        Formatted {
            div {
                class: "container content mt-4 has-text-centered",
                h1 { "Page not found" }
                p { "We are terribly sorry, but the page you requested doesn't exist." }
                p { "You can go back to the " a { href: "/", "home page" } "." },
                pre {
                    color: "red",
                    "log:\nattemped to navigate to: {route:?}"
                }
            }
        }
    }
}