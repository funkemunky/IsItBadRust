#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::DarkMode;
use log::info;

pub fn NavBar(cx: Scope) -> Element {
    let dark_mode = use_shared_state::<DarkMode>(cx).unwrap();

    let checked = dark_mode.read().0;

    cx.render(rsx! {
        nav {
            class: "navbar",
            div {
                class: "navbar-menu",
                a {
                    class: "navbar-item",
                    "Home"
                },
                div {
                    class: "navbar-end",
                    div {
                        class: "navbar-item",
                        input {
                            class: "switch is-info",
                            id: "darkMode",
                            checked: "{checked}",
                            r#type: "checkbox",
                            name: "darkMode",
                            oninput: |event| {
                                let set_dark_mode_to: bool = event.value.clone().parse().unwrap();
                                dark_mode.write().0 = set_dark_mode_to;

                                info!("Set darkmode to ${set_dark_mode_to}");
                            }
                        },
                        label {
                            r#for: "darkMode",
                            "Dark Mode"
                        }
                    }
                }
            }
        }
    })
}