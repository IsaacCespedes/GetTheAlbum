use crate::Route;
use dioxus::prelude::*;
use web_sys::console::log_1;

#[component]
pub fn TermsAgreementButton() -> Element {
    let navigator = use_navigator();

    let handle_submit = move |evt: Event<FormData>| {
        evt.prevent_default();
        evt.stop_propagation();
    };

    let handle_terms_click = move |_| {
        navigator.push(Route::Terms {});
    };

    rsx! {
      div { class: "terms-container",
        form {
          onsubmit: handle_submit,
          class: "terms-form",
          method: "javascript:void(0);",
          action: "javascript:void(0);",
          button { r#type: "button", onclick: handle_terms_click,
            i { class: "fas fa-gavel" }
            i { " " }
            "You legally agree not to sue (and other legal stuff (LOL))"
          }
        }
      }
    }
}
