use crate::components::{audio_player::AudioPlayer, Footer};
use dioxus::prelude::*;

const SONG_CSS: Asset = asset!("/assets/styling/song.css");

#[component]
pub fn Song27() -> Element {
    rsx! {
      document::Link { rel: "stylesheet", href: SONG_CSS }
      div { id: "song",
        h1 { "Lap Around The Sun" }
        p { "" }
        div { class: "song-details",
          p { "USCO Pending" }
          p { "BMI Work # 71691267" }
          p { "ISC # T-330.817.680-5" }
        }
      }
      AudioPlayer {
        src: "https://d1e6rbnlrwykmz.cloudfront.net/Lap+Around+the+Sun.mp3".to_string(),
        title: "Lap Around The Sun".to_string(),
      }
      div { style: "margin: 64px;",
        p { "Lyrics" }
        p { "It’s time to sing and dance and to have a lot of fun" }
        p { "Cause you won the whole race, ran a lap around the sun" }
        p { "It’s time to sing and dance and to have a lot of fun Aww yeaa" }
      }
      div { style: "margin: 64px;",
        p { "-" }
        p { "Yup, just the best birthday song ever" }
        p {
          "Pablo Arruda on Bass, Horacio París on Drums, Isaac on keys. Shout outs to Musiversal."
        }
        p { "Backstory and process details coming soon" }
      
      }
      Footer {}
    }
}
