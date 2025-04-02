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
        h1 { "Stay tuned for material updates" }
        p { "{email()}" }
        form {
          onsubmit: handle_submit,
          class: "newsletter-form",
          method: "javascript:void(0);",
          action: "javascript:void(0);",
          style: "display: flex; flex-direction: column; align-items: center; gap: 1rem;",
          div { style: "position: relative; width: 66%; display: flex; justify-content: center;",
            i {
              class: "fas fa-envelope",
              style: "position: absolute; left: 13px; top: 50%; transform: translateY(-50%); color: #666;",
            }
            input {
              r#type: "email",
              placeholder: "Email",
              value: "{email}",
              oninput: move |evt| email.set(evt.value()),
              required: true,
              style: "width: 100%; text-align: center;",
              disabled: true,
            }
          }

          button { r#type: "submit",
            // "Subscribe"
            // i { class: "mr-2", " " }
            i { class: "fas fa-signature" }
          }
        }
      }
    }
}
