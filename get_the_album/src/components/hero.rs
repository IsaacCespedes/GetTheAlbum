use crate::components::Echo;
use crate::Route;
use dioxus::prelude::*;

const ALBUM_ART: Asset = asset!(
    "/assets/images/album_art.jpg",
    ImageAssetOptions::new().with_avif()
);

#[component]
pub fn Hero() -> Element {
    rsx! {
        div { id: "hero", class: "w-60 max-w-xl mx-auto",
            h1 { class: "text-center mb-8", "Get The Album" }
            Echo {}
            div {
                id: "links",
                class: "flex justify-center",
                style: "margin-bottom: 24px;",
                h5 { style: "margin-bottom: 8px; text-align: center;",
                    "7 out of 21 + 2 out of 7. More coming soon..."
                }

                Link { to: Route::Song { id: 1 }, class: "",
                    "1. Intro - (Written) "
                    i { class: "fas fa-right-to-bracket text-black ml-2" }
                }

                div {
                    class: "text-gray-500",
                    style: "font-size: x-large; margin: 10px 0px; padding: 10px;",
                    "2. I`onGivaFuu - (EDM) "
                    i { class: "fas fa-school-lock text-black ml-2" }
                }
                div {
                    class: "text-gray-500",
                    style: "font-size: x-large; margin: 10px 0px; padding: 10px;",
                    "3. Let's Go Outside - (Jazz) "
                    i { class: "fas fa-school-lock text-black ml-2" }
                }
                Link {
                    to: Route::Song { id: 4 },
                    class: " bg-gray-300 text-gray-500 border border-gray-400 rounded px-4 py-2 m-1 hover:bg-gray-300",
                    "4. Lilly - (Hip Hop) "
                    i { class: "fas fa-right-to-bracket text-black ml-2" }
                }
                Link {
                    to: Route::Song { id: 5 },
                    class: " bg-gray-300 text-gray-500 border border-gray-400 rounded px-4 py-2 m-1 hover:bg-gray-300",
                    "5. Wisteria feat. Marta Garrett - (Classical Pop) "
                    i { class: "fas fa-right-to-bracket text-black ml-2" }
                }
                div {
                    class: "text-gray-500",
                    style: "font-size: x-large; margin: 10px 0px; padding: 10px;",
                    "6. The Lilacs - (Rock) "
                    i { class: "fas fa-school-lock text-black ml-2" }
                }
                Link {
                    to: Route::Song { id: 7 },
                    class: " bg-gray-300 text-gray-500 border border-gray-400 rounded px-4 py-2 m-1 hover:bg-gray-300",
                    "7. Mafioso Mo(u)rning - (Classical Rock) "
                    i { class: "fas fa-right-to-bracket text-black ml-2" }
                }
                div {
                    class: "text-gray-500",
                    style: "font-size: x-large; margin: 10px 0px; padding: 10px;",
                    "8. Anita - (Rock) "
                    i { class: "fas fa-school-lock text-black ml-2" }
                }
                Link {
                    to: Route::Song { id: 9 },
                    class: " bg-gray-300 text-gray-500 border border-gray-400 rounded px-4 py-2 m-1 hover:bg-gray-300",
                    "9. Fur Elise - (Classical Rock) "
                    i { class: "fas fa-right-to-bracket text-black ml-2" }
                }
                div {
                    class: "text-gray-500",
                    style: "font-size: x-large; margin: 10px 0px; padding: 10px;",
                    "10. Bear Claw - (Trap Rock) "
                    i { class: "fas fa-school-lock text-black ml-2" }
                }
                div {
                    class: "text-gray-500",
                    style: "font-size: x-large; margin: 10px 0px; padding: 10px;",
                    "11. Girl You're Infinity - (Country Pop) "
                    i { class: "fas fa-school-lock text-black ml-2" }
                }
                div {
                    class: "text-gray-500",
                    style: "font-size: x-large; margin: 10px 0px; padding: 10px;",
                    "12. Brought To The Surface - (Rock) "
                    i { class: "fas fa-school-lock text-black ml-2" }
                }
                div {
                    class: "text-gray-500",
                    style: "font-size: x-large; margin: 10px 0px; padding: 10px;",
                    "13. Letting Go - (Pop) "
                    i { class: "fas fa-school-lock text-black ml-2" }
                }
                div {
                    class: "text-gray-500",
                    style: "font-size: x-large; margin: 10px 0px; padding: 10px;",
                    "14. Something About You - (Funk) "
                    i { class: "fas fa-school-lock text-black ml-2" }
                }
                Link {
                    to: Route::Song { id: 15 },
                    class: " bg-gray-300 text-gray-500 border border-gray-400 rounded px-4 py-2 m-1 hover:bg-gray-300",
                    "15. Lightning Fire - (Rock) "
                    i { class: "fas fa-right-to-bracket text-black ml-2" }
                }
                div {
                    class: "text-gray-500",
                    style: "font-size: x-large; margin: 10px 0px; padding: 10px;",
                    "16. Can we talk? - (Classical) "
                    i { class: "fas fa-school-lock text-black ml-2" }
                }
                div {
                    class: "text-gray-500",
                    style: "font-size: x-large; margin: 10px 0px; padding: 10px;",
                    "17. Hope You're Ok - (Classical) "
                    i { class: "fas fa-school-lock text-black ml-2" }
                }
                div {
                    class: "text-gray-500",
                    style: "font-size: x-large; margin: 10px 0px; padding: 10px;",
                    "18. Wizard By Night - (Rock) "
                    i { class: "fas fa-school-lock text-black ml-2" }
                }
                Link {
                    to: Route::Song { id: 19 },
                    class: " bg-gray-300 text-gray-500 border border-gray-400 rounded px-4 py-2 m-1 hover:bg-gray-300",
                    "19. Lima To Sydney, Round-Trip - (Hip Hop) "
                    i { class: "fas fa-right-to-bracket text-black ml-2" }
                }
                div {
                    class: "text-gray-500",
                    style: "font-size: x-large; margin: 10px 0px; padding: 10px;",
                    "20. La tipa - (Reggaeton / Dembow) "
                    i { class: "fas fa-school-lock text-black ml-2" }
                }
                div {
                    class: "text-gray-500",
                    style: "font-size: x-large; margin: 10px 0px; padding: 10px;",
                    "21. Enemiga - (Bachata) "
                    i { class: "fas fa-school-lock text-black ml-2" }
                }
                h5 { style: "margin-bottom: 8px; text-align: center;", "Bonuses" }
                Link {
                    to: Route::Song { id: 22 },
                    class: " bg-gray-300 text-gray-500 border border-gray-400 rounded px-4 py-2 m-1 hover:bg-gray-300",
                    "Every Morning - (Afrobeat / Amapiano) "
                    i { class: "fas fa-right-to-bracket text-black ml-2" }
                }
                div {
                    class: "text-gray-500",
                    style: "font-size: x-large; margin: 10px 0px; padding: 10px;",
                    "About Time - (Pop) "
                    i { class: "fas fa-school-lock text-black ml-2" }
                }
                div {
                    class: "text-gray-500",
                    style: "font-size: x-large; margin: 10px 0px; padding: 10px;",
                    "Bad Babe Blues - (Blues) "
                    i { class: "fas fa-school-lock text-black ml-2" }
                }
                div {
                    class: "text-gray-500",
                    style: "font-size: x-large; margin: 10px 0px; padding: 10px;",
                    "Reflection - (Rock) "
                    i { class: "fas fa-school-lock text-black ml-2" }
                }
                div {
                    class: "text-gray-500",
                    style: "font-size: x-large; margin: 10px 0px; padding: 10px;",
                    "Jade Chess Set: The Chess Song - (Jingle) "
                    i { class: "fas fa-school-lock text-black ml-2" }
                }
                Link {
                    to: Route::Song { id: 27 },
                    class: " bg-gray-300 text-gray-500 border border-gray-400 rounded px-4 py-2 m-1 hover:bg-gray-300",
                    "Lap Around The Sun: The Birthday Song - (Swing) "
                    i { class: "fas fa-right-to-bracket text-black ml-2" }
                }
                div {
                    class: "text-gray-500",
                    style: "font-size: x-large; margin: 10px 0px; padding: 10px;",
                    "Symbology Infinity - (Hip Hop) "
                    i { class: "fas fa-school-lock text-black ml-2" }
                }
            }
            div { class: "flex justify-center mt-16 mb-8",
                div { class: "w-[300px] h-[400px] bg-gray-100 rounded-lg shadow-lg overflow-hidden",
                    img {
                        src: ALBUM_ART,
                        class: "w-[300px] h-[400px] object-cover",
                        alt: "Album Art",
                        style: "width: 300px !important; height: 400px !important;",
                    }
                }
            }
                // p { class: "text-center", "Album Art" }
        }
    }
}
