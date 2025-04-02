use crate::components::{audio_player::AudioPlayer, Footer};
use dioxus::prelude::*;

const SONG_CSS: Asset = asset!("/assets/styling/song.css");

#[component]
pub fn Song15() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: SONG_CSS }
        div { id: "song",
            h1 { "Lightning Fire" }
            p { "" }
            div { class: "song-details",
                p { "USCO Pending" }
                p { "BMI Work # 71691249" }
                p { "ISC # T-330.817.715-9" }
            }
            AudioPlayer {
                src: "https://d1e6rbnlrwykmz.cloudfront.net/Lightning+Fire.mp3".to_string(),
                title: "Lightning Fire".to_string(),
            }
            p { "Instrumental only" }
            p {
                "Isaac got in a few riffs here that have been in the pocket for a while. He would always go back to them when picking up his keytar."
            }
            p { "The Keytar Discovery Story is coming soon." }
            p { "The name Lightning Fire sounded cool, so that became the title." }
            p { "Shoutouts to:" }
            p { "Guilherme Medeiros, Luciano Vassão, and Taras Kuznetsov on the Production" }
            p { "André Vasconcellos on Bass" }
            p { "Christiano Galvão on Drums" }
            p { "Musiversal Platform for coordination and scheduling" }
            p { "Peace" }
        }
        Footer {}
    }
}
