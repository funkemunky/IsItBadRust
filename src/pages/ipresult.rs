use dioxus::core::Scope;
use dioxus::prelude::*;
use dioxus_bulma::prelude::*;
use log::info;
use crate::{Formatted, SharedKauriResponse};
use crate::utils::antivpn::get_ip_info;

#[inline_props]
pub fn IpResult(cx: Scope, ipToCheck: String) -> Element {
    if(ipToCheck.is_empty()) {
        return cx.render(rsx! {
            Formatted {
                Container {
                    p {
                        "Didn't have anything to input!"
                    }
                }
            }
        });
    }
    let response = use_shared_state::<SharedKauriResponse>(cx).unwrap();

    info!("Checking: {ipToCheck}");

    let api_check = use_future(cx, (), |_| get_ip_info(ipToCheck.clone()));

    match api_check.value() {
        Some(Ok(response)) => {
            info!("Response: {response}");
            let json = response;

            let success = match &json["success"].as_bool() {
                Some(success) => *success,
                None => false
            };

            if success {
                render! {
                Formatted {
                    Container {
                        div {
                            class: "panel mt-3",
                            p {
                                class: "panel-heading",
                                "{ipToCheck} Information"
                            },
                            table {
                            class: "table is-bordered is-centered is-fullwidth",
                         for (key, value) in json.as_object().unwrap() {
                            tr {
                                th {
                                    "{key}"
                                },
                                th {
                                    "{value}"
                                }
                            }
                         }
                    }
                        }
                    }
                }
            }
            } else {
                render! {
                    Formatted {
                        Container {
                            p {
                                "An error occurred while fetching an API request."
                            }
                        }
                    }
                }
            }
        },
        Some(Err(err)) => {
            render! {"An error occurred while fetching an API request: {err}"}
        },
        None => {
            render! {
                Formatted {
                    Container {
                        p {
                            "Loading..."
                        }
                    }
                }
            }
        }
    }
}