#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_bulma::prelude::*;
use log::info;

pub struct EnteredIp(pub String);

#[derive(Props)]
pub struct IpEntryProps<'a>  {
    #[props(default)]
    on_input: EventHandler<'a, FormEvent>,
    #[props(default)]
    on_entered: EventHandler<'a, EnteredIp>
}

pub fn IpEntry<'a>(cx: Scope<'a, IpEntryProps<'a>>) -> Element {
    let current_entering = use_state(cx, || "".to_string());

    cx.render(rsx! {
        form {
            input {
                class: "input",
                placeholder: "Enter an IP",
                oninput: move |event| {
                    current_entering.set(event.value.clone());
                    cx.props.on_input.call(event);
                }
            },
            Button {
                color: Colors::Primary,
                "Enter",
                onclick: |_| {
                    let current_input = current_entering.get().clone();

                    info!("Setting IP enter to {current_input}");
                    cx.props.on_entered.call(EnteredIp(current_input.clone()));
                    info!("Finished!");
                }
            }
        }
    })
}