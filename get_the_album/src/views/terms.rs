use crate::components::{Footer, TermsOfService};
use dioxus::prelude::*;

#[component]
pub fn Terms() -> Element {
    rsx! {
        div { class: "w-full",
            div { class: "container mx-auto px-4",
                TermsOfService {}
                Footer {}
            }
        }
    }
}
