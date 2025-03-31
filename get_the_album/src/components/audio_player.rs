use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct AudioPlayerProps {
    #[props(default)]
    src: String,
    #[props(default)]
    title: String,
}

#[component]
pub fn AudioPlayer(props: AudioPlayerProps) -> Element {
    rsx! {
        div { style: "margin-top: 64px; margin-bottom: 32px; display: flex; justify-content: center;",
            div {
                audio { controls: true, preload: "metadata",
                    source { src: props.src, r#type: "audio/mpeg" }
                    "Your browser does not support the audio element."
                }
                div {
                    div { class: "progress-bar" }
                }
            }
        }
    }
}
