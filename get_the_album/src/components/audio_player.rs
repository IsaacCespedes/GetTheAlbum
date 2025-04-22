use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct AudioPlayerProps {
    #[props(default)]
    src: String,
    #[props(default)]
    title: String,
    #[props(default = String::from("audio/mpeg"))]
    audio_type: String,
}

#[component]
pub fn AudioPlayer(props: AudioPlayerProps) -> Element {
    rsx! {
        div { style: "margin-top: 64px; margin-bottom: 32px; display: flex; justify-content: center;",
            div {
                audio { controls: true, preload: "metadata",
                    source { src: props.src, r#type: props.audio_type }
                    "Your browser does not support the audio element."
                }
                div {
                    div { class: "progress-bar" }
                }
            }
        }
    }
}
