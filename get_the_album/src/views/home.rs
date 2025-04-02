use crate::components::{Echo, Footer, Hero, Newsletter, TermsAgreementButton};
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        div { class: "home-container",
            Hero {}
            Newsletter {}
            Footer {}
        }
    }
}
