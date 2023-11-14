#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_bulma::layout::Container;
use log::{info};

use crate::{Formatted};
use crate::components::ipentry::IpEntry;

pub fn Home(cx: Scope) -> Element {
    let entered_state = use_state(cx, || "".to_string());

    cx.render(rsx! {
        Formatted {
            Container {
                IpEntry {
                    on_input: |event: FormEvent| {
                        let entered = event.data.value.clone();
    
                        info!("Entered input {entered}");
    
                        entered_state.set(entered);
                    }
                }
            }
        }
    })
}
