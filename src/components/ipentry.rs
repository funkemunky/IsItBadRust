#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::Route;
use log::{info};

pub struct EnteredIp(pub String);

#[derive(Props)]
pub struct IpEntryProps<'a>  {
    #[props(default)]
    on_input: EventHandler<'a, FormEvent>
}

pub fn IpEntry<'a>(cx: Scope<'a, IpEntryProps<'a>>) -> Element {
    let current_entering = use_shared_state::<EnteredIp>(cx)?;

    cx.render(rsx! {
        form {
            input {
                class: "input",
                placeholder: "Enter an IP",
                oninput: move |event| {
                    current_entering.write().0 = event.value.clone();
                    cx.props.on_input.call(event);
                }
            },
            Link {
                class: "button is-primary",
                to: {
                    let to_check = current_entering.read().0.clone();

                    info!("Sending to route {to_check}");
                    Route::IpResult {ipToCheck: to_check}
                },
                "Enter"
            }
        }
    })
}