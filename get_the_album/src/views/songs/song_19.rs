use crate::components::{audio_player::AudioPlayer, Footer};
use dioxus::prelude::*;

const SONG_CSS: Asset = asset!("/assets/styling/song.css");

#[component]
pub fn Song19() -> Element {
    rsx! {
      document::Link { rel: "stylesheet", href: SONG_CSS }
      div { id: "song",
        h1 { "Lima To Sydney, Round-Trip" }
        div { class: "song-details" }
        p { "USCO Pending" }
        p { "BMI Work # 71691251" }
        p { "ISC # T-330.817.717-0" }
        AudioPlayer {
          src: "https://d1e6rbnlrwykmz.cloudfront.net/Lima+To+Sydney+Round+Trip.mp3".to_string(),
          title: "Lima To Sydney, Round-Trip".to_string(),
        }
        p { style: "margin-top: 64px;", "Isaac had the lyrics, and some beatboxing." }
        p { "Minsoo Sung cheffed it up from there, shoutouts to Vedant Shankar on the mix" }
        p { "More backstory and process coming soon." }
        p { style: "margin-top: 64px;", "Official Lyrics" }
        p { "Shorty taller when she's sitting, know I mean" }
        p { "Walk round with it, like I won a tournament" }
        p { "Yeah I like em thick, in need of provision" }
        p { "know I got it covered, cause the kid's goin in" }
        p { "lookin into the eyes, and it speak a mile a minute" }
        p { "when she touch too, it be getting into kin-" }
        p { "knowing that I know, that she know what I know" }
        p { "winning on her own, hold the phone, take a pic" }
        p { "take it to Brazil, cause she look like one of them" }
        p { "or take the back out, outback like Sydney" }
        p { "propose preposition, yea the chick, she be with it" }
        p { "and at times irresponsive, I've been missin intermittent" }
        p { "tree breathing, while we're sittin tea sippin" }
        p { "Day one with it, while they walk in that mornin" }
        p { "Yea bed head, and I like mine while yawning" }
        p { "her jawn is no joke, but yea I get it" }
        p { "her cat made me a dog" }
        p { "she flowing for the log" }
        p { "the balls she makin boss" }
        p { "so shots are what she calls" }
        p { "like these punch lines how im hittin know i mean" }
        p { "walk around with it like it is an ornament" }
        p { "feeling like a king every time I Hopppawnit" }
        p { "with how i e8t learn rank from a bank" }
        p { "take it to honolulu we can keep it on a lo lo" }
        p { "or take it to Peru do it every afternuñez" }
        p { "no but really though she's the sight to be perusin" }
        p { "dang danger her love drugs real dank sh" }
        p { "she's the art was the thought when we MET" }
        p { "and that I immediately need to see me a vet" }
        p { "I bagged her cause her name rhyme now I'm in Annete" }
        p { "And yea it's cause I'm fly, and mine's via internet" }
        p { "saw the stacks with a face indifferent" }
        p { "ridin like amusement when i beat it like my music" }
        p { "wrecking records now, tripply rippin it" }
        p { "hoppin hate ribbittin it can go and keep it movin," }
        p { "get it?" }
        p { "her cat made me a dog," }
        p { "she flowing for the log" }
        p { "the balls she makin boss" }
        p { "so shots are what she calls" }
        p { "She threw it back in time, right" }
        p { "My face look like a lime...light" }
        p { "I'm betting on her fret, yea" }
        p { "And the baby I'm buy-in, aight? Get it." }
        p { "And yea it's cause I'm fly, and mine's via internet" }
        p { "saw the stacks with a face indifferent" }
        p { "ridin like amusement when i beat it like my music" }
        p { "wrecking records now, tripply rippin it" }
        p { "hoppin hate ribbittin it can go and keep it movin," }
        p { "get it?" }
        p { "her cat made me a dog," }
        p { "she flowing for the log" }
        p { "the balls she makin boss" }
        p { "so shots are what she calls" }
        p { "She threw it back in time, right" }
        p { "My face look like a lime...light" }
        p { "I'm betting on her fret, yea" }
        p { "And the baby I'm buy-in, aight? Get it." }
        p { "And yea it's cause I'm fly, and mine's via internet" }
        p { "saw the stacks with a face indifferent" }
        p { "ridin like amusement when i beat it like my music" }
        p { "wrecking records now, tripply rippin it" }
        p { "hoppin hate ribbittin it can go and keep it movin," }
        p { "get it?" }
        p { "her cat made me a dog," }
        p { "she flowing for the log" }
        p { "the balls she makin boss" }
        p { "so shots are what she calls" }
        p { "She threw it back in time, right" }
        p { "My face look like a lime...light" }
        p { "I'm betting on her fret, yea" }
        p { "And the baby I'm buy-in, aight? Get it." }
        p { "And yea it's cause I'm fly, and mine's via internet" }
        p { "saw the stacks with a face indifferent" }
        p { "ridin like amusement when i beat it like my music" }
        p { "wrecking records now, tripply rippin it" }
        p { "hoppin hate ribbittin it can go and keep it movin," }
        p { "get it?" }
        p { "her cat made me a dog," }
        p { "she flowing for the log" }
        p { "the balls she makin boss" }
        p { "so shots are what she calls" }
        p { "She threw it back in time, right" }
        p { "My face look like a lime...light" }
        p { "I'm betting on her fret, yea" }
        p { "And the baby I'm buy-in, aight? Get it." }
      }
      Footer {}
    }
}
