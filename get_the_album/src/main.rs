use dioxus::prelude::*;

use components::Navbar;
use views::{Blog, Comments, Home, Song, Terms};

mod components;
mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
    #[route("/song/:id")]
    Song { id: i32 },
    #[route("/comments")]
    Comments {},
    #[route("/terms")]
    Terms {},
}

const FAVICON: Asset = asset!("/assets/images/favicon.png");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // Build cool things ✌️

    rsx! {
        // Global app resources
        document::Link { rel: "icon", href: FAVICON }
        document::Link {
            rel: "stylesheet",
            href: "https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.5.1/css/all.min.css",
        }
        document::Link {
            rel: "stylesheet",
            href: "/assets/styles.css",
        }

        Router::<Route> {}
    }
}
