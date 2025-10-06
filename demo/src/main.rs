use dioxus::prelude::*;
use dioxus_emoji_picker::prelude::*;

const MAIN_CSS: Asset = asset!("/assets/main.css");
const EMOJI_PICKER_CSS: Asset = asset!("/assets/emoji_picker.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");
const HAPPY_ICON: Asset = asset!("/assets/happy.png");
const HAPPY_ICON_ACTIVE: Asset = asset!("/assets/happy_active.png");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        //document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: EMOJI_PICKER_CSS }
        Main {}
    }
}

#[component]
pub fn Main() -> Element {
	let mut show_emoji_picker = use_signal(|| false);
	let mut content = use_signal(|| 
		"Type here, click button below to bring up emoji picker 😄\n".to_string()
	);
	rsx! {
		div {
			class:"typing-area",
			form { 
				textarea {
					autofocus: "true",
					value: "{content}",
					oninput: move |evt| { 
						content.set(evt.value().clone());
					},
					onclick: move |_| { 
						show_emoji_picker.set(false);
					},
				}
			}
		}
		button {
			onclick: move |_| {
				show_emoji_picker.toggle()
			},
			{
				if *show_emoji_picker.read() {
					rsx! {
						img { src : "{HAPPY_ICON_ACTIVE}" }
					} 
				} else {
					rsx! {
						img { src : "{HAPPY_ICON}" }
					}
				}
			}
		}
		if show_emoji_picker() {
			div {
				EmojiPicker { content : content },
			}
		}
	}
}
