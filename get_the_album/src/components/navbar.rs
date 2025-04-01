use crate::Route;
use dioxus::prelude::*;

const NAVBAR_CSS: Asset = asset!("/assets/styling/navbar.css");
const ALBUM_ART: Asset = asset!(
    "/assets/images/album_art_drawing.png",
    ImageAssetOptions::new().with_avif()
);

#[component]
pub fn Navbar() -> Element {
    let current_route = use_route::<Route>();
    let is_comments_active = matches!(current_route, Route::Comments {});

    rsx! {
        document::Link { rel: "stylesheet", href: NAVBAR_CSS }
        document::Link { rel: "stylesheet", href: "/assets/styling/main.css" }
        document::Link { rel: "stylesheet", href: "/assets/tailwind.css" }

        nav { id: "navbar",
            Link { to: Route::Home {}, class: "home-link",
                img { src: ALBUM_ART, alt: "Home", class: "home-icon" }
            }
            Link {
                to: Route::Comments {},
                class: format!("blog-link{}", if is_comments_active { " active" } else { "" }),
                i { class: "fas fa-comments" }
                i { class: "mr-2", " " }
                "Comments"
            }
        }

        Outlet::<Route> {}
    }
}
