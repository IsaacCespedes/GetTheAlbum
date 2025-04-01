use crate::components::Footer;
use dioxus::prelude::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::window;

const COMMENTS_CSS: Asset = asset!("/assets/styling/comments.css");
const STORAGE_KEY: &str = "comments";
const COMMENTS_PER_PAGE: usize = 5;

#[derive(Clone, PartialEq, serde::Serialize, serde::Deserialize)]
struct Comment {
    id: i32,
    author: String,
    content: String,
    timestamp: String,
}

#[component]
pub fn Comments() -> Element {
    let mut comments = use_signal(Vec::new);
    let mut new_author = use_signal(String::new);
    let mut new_content = use_signal(String::new);
    let mut next_id = use_signal(|| 1);
    let mut is_loading = use_signal(|| true);
    let mut current_page = use_signal(|| 1);

    // Load comments from localStorage on component mount
    use_effect(move || {
        if let Some(window) = window() {
            if let Some(storage) = window.local_storage().unwrap() {
                match storage.get_item(STORAGE_KEY) {
                    Ok(Some(stored_comments)) => {
                        if let Ok(parsed_comments) =
                            serde_json::from_str::<Vec<Comment>>(&stored_comments)
                        {
                            comments.set(parsed_comments);
                            if let Some(max_id) = comments().iter().map(|c| c.id).max() {
                                next_id.set(max_id + 1);
                            }
                        }
                    }
                    Ok(None) => {
                        // No comments stored yet, initialize with empty vector
                        comments.set(Vec::new());
                        next_id.set(1);
                    }
                    Err(e) => {
                        // Error accessing localStorage (e.g., incognito mode)
                        comments.set(Vec::new());
                        next_id.set(1);
                    }
                }
                is_loading.set(false);
            }
        }
    });

    let handle_submit = move |_| {
        if !new_content().is_empty() && !new_author().is_empty() {
            // Set loading state before adding new comment
            is_loading.set(true);

            // Use a small delay to ensure the loading state is visible
            let window = window().unwrap();
            let window_clone = window.clone();
            let mut comments_clone = comments.clone();
            let mut new_author_clone = new_author.clone();
            let mut new_content_clone = new_content.clone();
            let mut next_id_clone = next_id.clone();
            let is_loading_clone = is_loading.clone();
            let mut current_page_clone = current_page.clone();

            let callback = Closure::wrap(Box::new(move || {
                let comment = Comment {
                    id: next_id_clone(),
                    author: new_author_clone(),
                    content: new_content_clone(),
                    timestamp: chrono::Local::now().format("%Y-%m-%d %H:%M").to_string(),
                };

                // Add the new comment to the beginning of the list
                let mut updated_comments = comments_clone();
                updated_comments.insert(0, comment.clone());
                comments_clone.set(updated_comments);

                next_id_clone += 1;
                new_author_clone.set(String::new());
                new_content_clone.set(String::new());
                // Reset to first page when adding new comment
                current_page_clone.set(1);

                // Save to localStorage
                if let Some(storage) = window_clone.local_storage().unwrap() {
                    if let Ok(json) = serde_json::to_string(&comments_clone()) {
                        match storage.set_item(STORAGE_KEY, &json) {
                            Ok(_) => {
                                web_sys::console::log_1(
                                    &"Successfully saved comments to localStorage".into(),
                                );
                            }
                            Err(e) => {
                                web_sys::console::log_1(
                                    &format!("Failed to save comments to localStorage: {:?}", e)
                                        .into(),
                                );
                            }
                        }
                    } else {
                        web_sys::console::log_1(&"Failed to serialize comments".into());
                    }
                } else {
                    web_sys::console::log_1(&"localStorage not available".into());
                }

                // Remove loading state after a longer delay for smoother transition
                let mut is_loading_final = is_loading_clone.clone();
                let callback = Closure::wrap(Box::new(move || {
                    is_loading_final.set(false);
                }) as Box<dyn FnMut()>);
                let _ = window_clone.set_timeout_with_callback_and_timeout_and_arguments_0(
                    callback.as_ref().unchecked_ref(),
                    800, // Increased delay for smoother transition
                );
                callback.forget();
            }) as Box<dyn FnMut()>);

            let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                callback.as_ref().unchecked_ref(),
                100, // Slightly longer initial delay
            );
            callback.forget();
        }
    };

    // Calculate pagination values
    let total_pages = (comments().len() as f64 / COMMENTS_PER_PAGE as f64).ceil() as usize;
    let start_idx = (current_page() - 1) * COMMENTS_PER_PAGE;
    let end_idx = (start_idx + COMMENTS_PER_PAGE).min(comments().len());

    // Create a longer-lived value for the comments
    let comments_vec = comments();
    let paginated_comments = comments_vec.iter().skip(start_idx).take(COMMENTS_PER_PAGE);
    let remaining = COMMENTS_PER_PAGE - paginated_comments.clone().count();

    rsx! {
      document::Link { rel: "stylesheet", href: COMMENTS_CSS }
      div { class: "comments-container",
        // Comment form
        div { class: "comment-form",
          div { class: "mb-4",
            input {
              r#type: "text",
              placeholder: "Your name",
              value: "{new_author}",
              oninput: move |evt| new_author.set(evt.value()),
              class: "w-full p-2 border rounded",
              required: true,
            }
          }
          div { class: "mb-4",
            textarea {
              placeholder: "Your comment",
              value: "{new_content}",
              oninput: move |evt| new_content.set(evt.value()),
              class: "w-full p-2 border rounded",
              rows: "4",
              required: true,
            }
          }
          if is_loading() {
            div { class: "loading-spinner" }
          } else {
            button {
              r#type: "button",
              onclick: handle_submit,
              class: "bg-blue-500 text-white px-4 py-2 rounded hover:bg-blue-600",
              i { class: "mr-2", "Comment" }
              i { class: "mr-2", " " }
              i { class: "fas fa-comment" }
            }
          }
        }


        // Comments list
        div { class: "comments-list",
          // Show actual comments
          for comment in paginated_comments {
            div { class: "comment",
              div { class: "comment-header",
                h3 { class: "comment-author", "{comment.author}" }
                span { class: "comment-timestamp", "{comment.timestamp}" }
              }
              p { class: "comment-content", "{comment.content}" }
            }
          }
          // Fill remaining space with placeholder comments
          for _ in 0..remaining {
            div { class: "comment placeholder",
              div { class: "comment-header",
                div { class: "comment-author" }
                div { class: "comment-timestamp" }
              }
              div { class: "comment-content" }
            }
          }
        }

        // Pagination controls
        if total_pages > 1 {
          div { class: "pagination",
            button {
              class: "pagination-button",
              disabled: current_page() == 1 || is_loading(),
              onclick: move |_| current_page -= 1,
              i { class: "fas fa-arrow-left" }
              i { class: "mr-2", " " }
              "Previous"
            }
            span { class: "pagination-info", "Page {current_page} of {total_pages}" }
            button {
              class: "pagination-button",
              disabled: current_page() == total_pages || is_loading(),
              onclick: move |_| current_page += 1,
              "Next"
              i { class: "mr-2", " " }
              i { class: "fas fa-arrow-right ml-3" }
            }
          }
        }
      }
      Footer {}
    }
}
