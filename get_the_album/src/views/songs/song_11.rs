use crate::components::{audio_player::AudioPlayer, Footer};
use dioxus::prelude::*;

const SONG_CSS: Asset = asset!("/assets/styling/song.css");

#[component]
pub fn Song11() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: SONG_CSS }
        div { id: "song",
            h1 { "Girl You're Infinity" }
            p { "" }
            div { class: "song-details",
                p { "USCO Pending" }
                p { "BMI Pending" }
            }
        }
        AudioPlayer {
            src: "https://d1e6rbnlrwykmz.cloudfront.net/Girl+You're+Infinity.mp3".to_string(),
            title: "Girl You're Infinity".to_string(),
        }
        div { style: "margin: 64px; text-align: center; display: flex; flex-direction: column; align-items: center;",
            p { "© 2025" }
            p { "More details coming soon..." }
        }
        Footer {}
    }
}
