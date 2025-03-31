use dioxus::prelude::*;

#[component]
pub fn Footer() -> Element {
    rsx! {
      footer {
        class: "bg-gray-900 text-white py-12 w-full",
        style: "margin-top: 128px; margin-bottom: 64px; text-align: center;",
        div { class: "max-w-4xl mx-auto px-4",
          div { class: "grid grid-cols-1 md:grid-cols-2 gap-8",
            // Left column
            div { class: "flex flex-col items-center",
              h3 { class: "text-xl font-bold mb-4", "Get The Album LLC" }
              h3 { class: "text-xl font-bold mb-4", "Get The Album Publishing" }
              h3 { class: "text-xl font-bold mb-4", "getthealbum@protonmail.com" }
            
            }
            // Right column
            div { class: "flex flex-col items-center",
              p { "© 2025 getthealbum.com - Get The Album - All rights reserved" }
            }
          }
        
        }
      }
    }
}
