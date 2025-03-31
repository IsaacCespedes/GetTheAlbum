use crate::Route;
use dioxus::prelude::*;

const NAVBAR_CSS: Asset = asset!("/assets/styling/navbar.css");
const ALBUM_ART: Asset = asset!(
    "/assets/images/album_art_drawing.png",
    ImageAssetOptions::new().with_avif()
);

#[component]
pub fn Navbar() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: NAVBAR_CSS }
        document::Link { rel: "stylesheet", href: "/assets/styling/main.css" }
        document::Link { rel: "stylesheet", href: "/assets/tailwind.css" }

        nav { id: "navbar",
            Link { to: Route::Home {}, class: "home-link",
                img { src: ALBUM_ART, alt: "Home", class: "home-icon" }
            }
            Link { to: Route::Blog { id: 1 }, class: "blog-link", "Blog" }
        }

        Outlet::<Route> {}
    }
}
