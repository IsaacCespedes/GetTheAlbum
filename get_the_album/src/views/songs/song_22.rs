use crate::components::{audio_player::AudioPlayer, Footer};
use dioxus::prelude::*;

const SONG_CSS: Asset = asset!("/assets/styling/song.css");

#[component]
pub fn Song22() -> Element {
    rsx! {
      document::Link { rel: "stylesheet", href: SONG_CSS }
      div { id: "song",
        h1 { "Every Morning - (Afrobeats / Amapiano)" }
        p { "prod. by EeFlat" }
        p { "USCO Pending" }
        p { "BMI Work Number: 71691127" }
      }
      AudioPlayer {
        src: "https://d1e6rbnlrwykmz.cloudfront.net/Every+Morning.mp3".to_string(),
        title: "Every Morning".to_string(),
      }
      div {
        p { style: "margin-top: 64px;", "Shout out to @eeflat the tunes god!" }
        p {
          "All Isaac gave him was some vocalizations, lyrics, and some basic melody lines and he took it flying from there."
        }
        p {
          "This was supposed to be a 16 bar, but the Afrobeats style is too much fun and it got to 64, so that was cool."
        }
        p {
          "Definitely some trial and error with getting the sound right, probably the most challenging style for Isaac. That, and singing on a higher key than usual."
        }
        p {
          "EFlat was the one with the idea to cap the song off with the “Whine, I said make you whine to the rhythm and dance…” "
        }
        p {
          "Very natural to groove with, and it brought out a new tonal quality and break in rhythm."
        }
        p { "You already know. " }
        p { "The background story is pretty interesting…stay tuned for that." }
        p { style: "margin-top: 64px;", "Official Lyrics" }
        p { "Yes we tease with the phonin' yes" }
        p { "but we real, we no phoniness" }
        p { "we're the way that we always been" }
        p { "though they're awed, in amazement yea" }
        p { "we are hot what about it yea" }
        p { "It is love not just horniness" }
        p { "though we moan every morning yes" }
        p { "Making more every evening yes" }
        p { "yes boss babe you get on it yea" }
        p { "its on me but you own it yea" }
        p { "It is love not unloneliness" }
        p { "though with you feel surrounded yea" }
        p { "everybody they notice yea" }
        p { "yea you got them on notice yea" }
        p { "and the tings is that you know this yea" }
        p { "powerful, run for potus yes" }
        p { "you a lion no lyin ye" }
        p { "when you win we are tied o ye" }
        p { "you be drugs so enlightenin'" }
        p { "knock me out without fightin ye" }
        p { "baby cub you a pouncer ye" }
        p { "we a club you a bouncer ye" }
        p { "so let's keep it one thousand" }
        p { "times one thousand allowance yee" }
        p { "body speaks you the type oh ye" }
        p { "so lets sign on the line oh ye" }
        p { "make men mine, make you mine oh ye" }
        p { "you make water from wine oh yea" }
        p { "we can bring in a bible yea" }
        p { "a big balance I'm stable yea" }
        p { "what you want for your bridal yea" }
        p { "I want you with 2 navels yea" }
        p { "- " }
        p { "Whine, I said make you whine to the rhythm and dance…" }
        p { "I said make you whine to the rhythm and dance…" }
        p { "I said make you whine to the rhythm and dance…" }
        p { "y-y-y-y-yea" }
      }
      Footer {}
    }
}
