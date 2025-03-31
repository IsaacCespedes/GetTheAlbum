use crate::views::songs::Song as SongComponent;
use dioxus::prelude::*;

const SONG_CSS: Asset = asset!("/assets/styling/song.css");

#[component]
pub fn Song(id: i32) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: SONG_CSS }
        SongComponent { id }
    }
}
