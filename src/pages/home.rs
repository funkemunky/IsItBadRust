#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::components::ipentry::IpEntry;


pub fn Home(cx: Scope) -> Element {
    cx.render(rsx! {
        IpEntry {}
    })
}
