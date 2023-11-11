#![allow(non_snake_case)]

use dioxus::prelude::*;

pub fn NavBar(cx: Scope) -> Element {
    cx.render(rsx! {
        nav {
            class: "navbar is-dark",
            div {
                class: "navbar-menu",
                a {
                    class: "navbar-item",
                    "Home"
                }
            }
        }
    })
}