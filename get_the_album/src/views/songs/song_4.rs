use crate::components::{audio_player::AudioPlayer, Footer};
use dioxus::prelude::*;

const SONG_CSS: Asset = asset!("/assets/styling/song.css");

#[component]
pub fn Song4() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: SONG_CSS }
        div { id: "song",
            h1 { "Lilly" }
            p { "" }
            div { class: "song-details",
                p { "US Copyright Office Registration No. PA 2-513-946" }
                p { "BMI Work # 71670019" }
                p { "ISWC T-330.819.002-5" }
            }
        }
        AudioPlayer {
            src: "https://d1e6rbnlrwykmz.cloudfront.net/Lilly.mp3".to_string(),
            title: "Lilly".to_string(),
        }
        div { style: "margin: 64px;",
            p { "Robert Lotreck on the track you already know." }
            p { "Isaac composed every layer of this one." }
            p { "Robert produced and layered drum sounds over Isaac's beatboxing." }
            p {
                "Isaac was close to buying a software to create the drum sound to follow the beatboxing, "
            }
            p { "however, the software  was AI,  so this layering was done manually." }
            p { "For the full story behind this one… stay tuned." }
        }

        div { style: "margin: 64px;",
            h2 { "Lyrics" }
            p { "Intro" }
            p { "You know what time it is" }
            p { "This is the bx ny baby" }
            p { "You better believe it" }
            p { "You have a voice" }
            p { "Music is not a schedule 1 drug you hear?" }
            p { "So yeaa lets do this" }
            p { "The name is I. S. double A" }
            p { "Like my ladies lovely" }
            p { "I like me sippin bubbly" }
            p { "The ions from the struggle eh" }
            p { "The iron from the rubble eh" }
            p { "On the toes like relevé" }
            p { "All the while stayin relevan-" }
            p { "See, I'm just me" }
            p { "Fresh out the fryer" }
            p { "Flyer than a fleet" }
            p { "Bronx borough on the beat" }
            p { "Let me give it straight and neat" }
            p { "While I talk about Lilly" }
            p { "Because she is a keeper" }
            p { "And I'm used to rentin" }
            p { "And I've seen it all, but none of this here in a minute" }
            p { "In the midst of all of the inny minny" }
            p { "She been talkin to my face and she's been talkin kids" }
            p { "She ain't judgin like these bigwigs givin run on sentences" }
            p { "Appreciate it, around my way they say I fux with it" }
            p { "Yeah that's word to government" }
            p { "On the mind like Huberman" }
            p { "For the way she's out here, out manueverin" }
            p { "Making moves like a movement" }
            p { "Man I feel i won so hard... I'm losin it" }
            p { "Half hook" }
            p { "She's a babe but the girl is grown" }
            p { "Get the point, it's no game, no" }
            p { "She's a type to give it all life long" }
            p { "So meaning In the best way," }
            p { "Know meaning in the best way" }
            p { "On many days dodged many strays, most were cupid's" }
            p { "Mother prayed to Jesus to keep me from Judas" }
            p { "Resolvin all the problems it made me a rubix" }
            p { "It had me changing ways, and I asked me who this" }
            p { "For real though, 33, so I count thanksgivings" }
            p { "If you know you know exactly what I'm meaning" }
            p { "Been wrestling some demons but she is a christian" }
            p { "You know it's really real when it's real specific" }
            p { "Pick an ocean, we can go, make dough, and chill" }
            p { "Just a call or text, there's no need for digits" }
            p { "This is different from those chicks just seeing wishlists" }
            p { "Cause back then I was broke, and no, I don't miss it" }
            p { "And that's a need to know when we're talkin Mrs." }
            p { "And that's just how I be, I just mind my business" }
            p { "And she's just the type, shackle like a wristlet" }
            p { "So if you like this song give your thanks to Lilly" }
            p { "Half hook" }
            p { "She's a babe but the girl is grown" }
            p { "Get the point, it's no game, know" }
            p { "She's a type to give it all lifelong" }
            p { "So meaning In the best way," }
            p { "Know meaning in the best way" }
            p { "Stacked, and she meets her ends" }
            p { "Making bands in business pants" }
            p { "So believe in God cause damn" }
            p { "Meaning in the best way," }
            p { "Know meaning in the best way" }
            p { "Outro" }
            p { "Bronx ny" }
            p { "You already know what time it is man" }
            p { "Bars" }
            p { "Bars bars uhh bars" }
            p { "Hit with the keys, hit it with the beats. you know what I mean" }
            p { "With the lyrics" }
            p { "You know what I mean" }
            p { "I sing it you know what i mean" }
            p { "Girl's got me writing stuff man" }
            p { "You know how it is" }
            p { "Get you crazy get on your mind and stuff man" }
            p { "...." }
            p { "You feel me" }
            p { "You know how it is" }
            p { "(and you just wanna make music" }
            p { "All you gotta do is take 3 fingers" }
            p { "Thumb, pinky, and middle" }
            p { "You know what I mean" }
            p { "Curse whatever gotta be cursed and put them back n forth on the keyboard man" }
            p { "And say something about something" }
            p { "You know what I mean" }
            p { "I know that you know that I know you know what I mean" }
            p { "So peace" }
        }
        Footer {}
    }
}
