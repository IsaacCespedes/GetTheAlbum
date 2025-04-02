use dioxus::prelude::*;

#[component]
pub fn TermsButton() -> Element {
    rsx! {
        div { class: "flex justify-center mt-8",
            button {
                class: "px-6 py-3 text-sm font-medium text-gray-600 hover:text-gray-800 transition-colors duration-200",
                onclick: move |_| {
                    // TODO: Implement terms of service navigation
                },
                "Terms of Service"
            }
        }
    }
}
