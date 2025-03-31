use dioxus::prelude::*;
use web_sys::console::log_1;

#[component]
pub fn Newsletter() -> Element {
    let mut email = use_signal(String::new);

    let handle_submit = move |evt: Event<FormData>| {
        evt.prevent_default();
        evt.stop_propagation();
        // TODO: Implement newsletter signup logic
        log_1(&format!("Newsletter signup for: {}", email()).into());
        // email.set(String::new());
    };

    rsx! {
      div { class: "newsletter-container",
        h1 { "Stay Updated" }
        p { "{email()}" }
        p { "Subscribe to our newsletter for the latest updates and exclusive content." }
        form {
          onsubmit: handle_submit,
          class: "newsletter-form",
          method: "javascript:void(0);",
          action: "javascript:void(0);",
          input {
            r#type: "email",
            placeholder: "Enter your email",
            value: "{email}",
            oninput: move |evt| email.set(evt.value()),
            required: true,
          }
          button { r#type: "submit", "Subscribe" }
        }
      }
    }
}
