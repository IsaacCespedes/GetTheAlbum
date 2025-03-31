use crate::components::{audio_player::AudioPlayer, Footer};
use dioxus::prelude::*;

const SONG_CSS: Asset = asset!("/assets/styling/song.css");

#[component]
pub fn Song9() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: SONG_CSS }
        div { id: "song",
            h1 { "Fur Elise Rock" }
            p { "" }
            div { class: "song-details",
                p { "USCO Pending" }
                p { "BMI Work # 71691198" }
                p { "ISC # T-331.358.010-4" }
            }
        }
        AudioPlayer {
            src: "https://d1e6rbnlrwykmz.cloudfront.net/Fur+Elise+Rock.mp3".to_string(),
            title: "Fur Elise Rock".to_string(),
        }
        div { style: "margin: 64px;",
            p { "© 2025" }
            p { "(Instrumental only)" }
            p { style: "font-weight: bold;", "The Process" }
            p { "The drop was not planned at all. That was decided in a split second." }
            p {
                "And Isaac just now, while writing this, realized it was at the 00:42 mark. Numerology."
            }
            p {
                "It was probably just a small posture change, or something from the lead up to that point that begged the question of how keep it going and pick up the pace, "
            }
            p {
                "and Isaac had just enough music IQ to figure out how to make that happen in that moment."
            }
            p {
                "They decided that they could not recreate that playthrough, that it would not be as authentic."
            }
            p {
                "So the audio was stripped from the video, Robert added some effects, and there you have it."
            }
            p { style: "font-weight: bold;", "Backstory" }
            p { "Robert Lotreck on the Drums. LoTreck Music." }
            p { "Isaac on the keytar. You already knew." }
            p { "This is probably a good time to give the story of how Isaac and Robert met." }
            p { "As you know from some of these tracks, Isaac is from NYC, Bronx." }
            p {
                "After a few years of working programming jobs, he signed up to be a mentor for a web development program hosted by Columbia University."
            }
            p { "Some time went by, probably close to a year…" }
            p {
                "And one day Isaac was looking for events in the city, and there was this Columbia event."
            }
            p { "It was for alumni, but they welcomed anyone else who was looking to network." }
            p {
                "Isaac gave himself a pass to crash the event since he had the mentorship program experience, even though he went to Stony Brook."
            }
            p { "So yea, that’s how they met." }
            p { "Months later, Isaac had the idea to promote an e-commerce project he had going on" }
            p { "and the idea was to video record some jam sessions." }
            p { "A few of the jams were tracks that will be on this album," }
            p { "but this one is the only one where the original recording stayed." }
            p { "– Isaac" }
        }
        Footer {}
    }
}
