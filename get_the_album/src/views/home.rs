use crate::components::{Echo, Footer, Hero, Newsletter};
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        div { class: "w-full",
            div { class: "container mx-auto px-4",
                Hero {}
                Newsletter {}
                Footer {}
            }
        }
    }
}
