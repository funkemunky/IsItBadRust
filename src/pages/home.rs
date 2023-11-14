#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_bulma::layout::Container;
use log::info;

use crate::components::ipentry::IpEntry;

use crate::utils::antivpn::*;

pub fn Home(cx: Scope) -> Element {
    let entered_state = use_state(cx, || "".to_string());
    let future = move |_| {
        cx.spawn({
            let entered_state = entered_state.to_owned();

            async move {
                let kauriResponse = match get_ip_info(entered_state.get().to_string()).await {
                    Ok(response) => response,
                    Err(error) => panic!("Shit")
                };

                info!("Response: {kauriResponse:?}");
            }
        });
    };

    cx.render(rsx! {
        Container {
            IpEntry {
                on_input: |event: FormEvent| {
                    let entered = event.data.value.clone();

                    info!("Using entered {entered}");


                    entered_state.set(entered);
                },
                on_entered: future
            }
        }
    })
}
