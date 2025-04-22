mod song_1;
mod song_11;
mod song_15;
mod song_19;
mod song_2;
mod song_22;
mod song_26;
mod song_27;
mod song_4;
mod song_5;
mod song_6;
mod song_7;
mod song_9;

use dioxus::prelude::*;
use song_1::Song1;
use song_11::Song11;
use song_15::Song15;
use song_19::Song19;
use song_2::Song2;
use song_22::Song22;
use song_26::Song26;
use song_27::Song27;
use song_4::Song4;
use song_5::Song5;
use song_6::Song6;
use song_7::Song7;
use song_9::Song9;

#[component]
pub fn Song(id: i32) -> Element {
    match id {
        1 => rsx! {
          Song1 {}
        },
        2 => rsx! {
          Song2 {}
        },
        4 => rsx! {
          Song4 {}
        },
        5 => rsx! {
          Song5 {}
        },
        6 => rsx! {
          Song6 {}
        },
        7 => rsx! {
          Song7 {}
        },
        9 => rsx! {
          Song9 {}
        },
        11 => rsx! {
          Song11 {}
        },
        15 => rsx! {
          Song15 {}
        },
        19 => rsx! {
          Song19 {}
        },
        22 => rsx! {
          Song22 {}
        },
        26 => rsx! {
          Song26 {}
        },
        27 => rsx! {
          Song27 {}
        },
        _ => rsx! {
          div {
            h1 { "Song not found" }
            p { "The requested song could not be found." }
          }
        },
    }
}
