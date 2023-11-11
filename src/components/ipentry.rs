#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_bulma::prelude::*;
use log::info;

pub struct EnteredIp(pub String);

pub fn IpEntry(cx: Scope) -> Element {
    let entered_ip_result = use_shared_state::<EnteredIp>(cx).unwrap();

    let current_entering = use_state(cx, || "".to_string());

    cx.render(rsx! {
        form {
            input {
                class: "input",
                placeholder: "Enter an IP",
                oninput: |event| {
                    let entered = event.value.clone();
                    info!("Entered ${entered}");
                    current_entering.set(entered);
                }
            },
            Button {
                color: Colors::Primary,
                "Enter",
                onclick: |_| {
                    let current_input = current_entering.get().clone();

                    info!("Setting IP enter to ${current_input}");
                    entered_ip_result.write().0 = current_input;
                    info!("Finished!");
                }
            }
        }
    })
}