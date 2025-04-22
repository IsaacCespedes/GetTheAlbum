use crate::components::{audio_player::AudioPlayer, Footer};
use dioxus::prelude::*;

const SONG_CSS: Asset = asset!("/assets/styling/song.css");

#[component]
pub fn Song2() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: SONG_CSS }
        div { id: "song",
            h1 { "I'onGivaFuu" }
            p { "" }
            div { class: "song-details",
                p { "USCO Pending" }
                p { "BMI Pending" }
            }
        }
        AudioPlayer {
            src: "https://d1e6rbnlrwykmz.cloudfront.net/IonGivaFuu.wav".to_string(),
            title: "I'onGivaFuu".to_string(),
            audio_type: "audio/wav".to_string(),
        }
        div { style: "margin: 64px; text-align: center; display: flex; flex-direction: column; align-items: center;",
            p { "© 2025" }
            p { "More details coming soon..." }
        }
        Footer {}
    }
}
