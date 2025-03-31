use crate::components::{audio_player::AudioPlayer, Footer};
use dioxus::prelude::*;

const SONG_CSS: Asset = asset!("/assets/styling/song.css");

#[component]
pub fn Song7() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: SONG_CSS }
        div { id: "song",
            h1 { "Mafioso Mourning" }
            p { "" }
            div { class: "song-details",
                p { "USCO Pending" }
                p { "BMI Pending" }
            }
        }
        AudioPlayer {
            src: "https://d1e6rbnlrwykmz.cloudfront.net/Mafioso+Mourning.mp3".to_string(),
            title: "Mafioso Mourning".to_string(),
        }
        div { style: "margin: 64px;",
            p { "© 2025" }
            p { "Instrumental only." }
            p {
                "Almost switched to purely classical, but the original listing did say classical rock and so here it is!"
            }
            p { "Pablo Arruda on Bass, Horracio Paris on Drums" }
            p { "Luciano Vassão, Gabor Udvarhelyi and Yago Marques on Production" }
            p {
                "Like most instrumental work, this was recorded with a Roland AX Edge keytar and a Focusrite Solo."
            }
            p { "More details on home setup coming soon." }
            p {
                "As for backstory… like in most cases, Isaac was just trying different riffs out on the keys when he stumbled on the arpeggio as heard in the first few notes here."
            }
            p {
                "The rest kind of just flowed from there. What would be interesting is if this included a deeper dive into the music theory."
            }
            p {
                "That alone could take a session bookings, because right now the analysis doesn’t go beyond descending arpeggios and just feeling out where the tune wants to go."
            }
            p {
                "As far as structure goes, maybe it could have just been an AAB or AABAB section format instead of the AABAAB that it is now."
            }
            p {
                "The title came from the inspiration of The Godfather movie, since it has a classical theme music track."
            }
            p {
                "Around that time, close to a year ago, Isaac experienced a close passing away as well."
            }
            p { "After many takes of this piece, from start to end, Isaac sent it over for production." }
            p { "That’s how the old movie tonal effect was integrated." }
        }
        Footer {}
    }
}
