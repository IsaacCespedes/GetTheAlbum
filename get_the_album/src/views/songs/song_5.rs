use crate::components::{audio_player::AudioPlayer, Footer};
use dioxus::prelude::*;

const SONG_CSS: Asset = asset!("/assets/styling/song.css");

#[component]
pub fn Song5() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: SONG_CSS }
        div { id: "song",
            h1 { "Wisteria" }
            p { "" }
            div { class: "song-details",
                p { "USCO Pending" }
                p { "BMI Work # 71691237" }
                p { "ISWC T-330.817.719-3" }
            }
        }
        AudioPlayer {
            src: "https://d1e6rbnlrwykmz.cloudfront.net/Wisteria.mp3".to_string(),
            title: "Wisteria".to_string(),
        }
        div { style: "margin: 64px;",
            p { "Shoutouts to :" }
            p { "Marta Garrett on vocals" }
            p { "Davy Bergier on guitar" }
            p { "João Paulo Drumond on Percussions" }
            p { "Luciano Vassão on mastering" }
            p { "Federico Hencker on the mix" }
            p { "Alex Schindler and Guilherme Medeiros on production" }
            p { "and the Musiversal platform" }
            p { "Composer, author, vocals, and keys: Isaac, you already knoww" }
            p { "Stay tuned for the backstory…it’s pretty good" }
            p { style: "margin-top: 64px;", "Official Lyrics" }
            p { "She's bad, her hurt is worse" }
            p { "the things she don't deserve" }
            p { "bachelors and two masters" }
            p { "ye heard what I said yurr" }
            p { "Round' way in the bronx borough" }
            p { "On God, take it to church" }
            p { "Gets green, and down to earth" }
            p { "and so she's swerving curves" }
            p { "© 2025" }
        }
        Footer {}
    }
}
