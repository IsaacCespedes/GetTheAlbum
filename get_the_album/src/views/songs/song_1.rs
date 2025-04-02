use crate::components::Footer;
use dioxus::prelude::*;

const SONG_CSS: Asset = asset!("/assets/styling/song.css");

#[component]
pub fn Song1() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: SONG_CSS }
        div { id: "song",
            h1 { "..." }
            p { "Every line, I write, I sign, I copyright" }
            p { "You know what I'm saying, so ya capisce right?" }
            p { "The style's original, this no copy, right?" }
            p { "Winning like un domino capi, right?" }
            p { "Yea the plan was to entitle that one capiright," }
            p { "but I might just switch it to fuck this AI aight?" }
            p { "if you know you know" }
            p { "the style's origin in all" }
            p { "I'm not robot food" }
            p { "This know..." }
        }

        div { style: "margin: 64px;",
            p { "A cute 4+2 or 5+1 bar, depending on how you look at it" }
            p {
                "Isaac can find the exact day and time he started writing this piece, and where he was."
            }
            p {
                "He just now made the choice to start writing in the third person, while writing this, and he is running with it."
            }
            p {
                "Also, he is running with Pill Breakfast as an alias / stage name. Isaac aka Pill Breakfast. There will also be a third name, probably to be announced soon."
            }
            p { style: "margin-top: 32px;",
                "While on the way to an open mic, the first words were on the pad, minutes before the notification:"
            }
            p { "The feds launched an investigation into corporate AI, over copyright infringment." }
            p { "(Maybe the notes app he uses has a history feature for proof.)" }
            p { "At the time, he had recorded and copyrighted the tracks" }
            p { "This is what inspired the first few lines." }
            p {
                "In the following weeks was when he got the idea to write the lines in different languages."
            }
            p { "The second section was added around that time also." }
            p {
                "On multiple occasions, people would ask him if he was the author or singer when he shared his songs."
            }
            p { "There is no AI in any layer of any track for this album." }
            p {
                "If a thorough investigation is needed, the author will go to every end to prove authenticity."
            }
            p {
                "He made sure to search for the right people to fill in any blanks in his musical production ability."
            }
            p {
                "He never asked anyone to write a single lyric for him, no ghostwriters, and he takes great pride in that."
            }
            p { "Every lyric in this album was written by yours truly." }
            p { "\"Every line I write, I sign, I copyright.\"" }
            p { "There were 3 times where he ALMOST used AI." }
            p { "They will explained along with each song as they come out." }
            p { "(Anita, Lilly, and Brought to the Surface)." }
            p {
                "This is not to say that he will never use AI. Maybe you want to see an AI bonus track."
            }
            p { "But right now, the mode is weapons down, gloves off." }
            p { "Note: The website features are being built with the help of AI..." }
            p {
                "...however, that strictly on the coding side, so the systems can be as fully featured as possible. Stay tuned."
            }
        }
        Footer {}
    }
}
