use dioxus::prelude::*;
use dioxus_emoji_picker::prelude::*;
use emojis::*;

const MAIN_CSS: Asset = asset!("/assets/main.css");
const EMOJI_PICKER_CSS: Asset = asset!("/assets/emoji_picker.css");

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
fn LightDark() -> Element {

	rsx! {
		div {
			"Dark mode:"
		}
		form {
			class: "dark-light",
			input {
				type : "radio",
				id : "auto",
				value : "auto",
				name : "dark-light",
			}
			label {
				for: "auto",
				"Auto"
			}
			input {
				type : "radio",
				id : "dark",
				value : "dark",
				name : "dark-light",
			}
			label {
				for: "dark",
				"Dark"
			}
			input {
				type : "radio",
				id : "light",
				value : "light",
				name : "dark-light",
			}
			label {
				for: "light",
				"Light"
			}
		}
	}
}

#[component]
pub fn Main() -> Element {
	let mut emoji = use_signal(|| None);
	rsx! {
		div {
			class : "top",
			h1 {
				"dioxus_emoji_picker"
			}
			p {
				"Emoji Picker for "
				a {
					href : "https://dioxuslabs.com/",
					"Dioxus Web Framework"
				}
			}
			div {
				class: "demo",
				div {
					class: "lhs",
					EmojiPicker { 
						emoji : emoji,
						options : EmojiPickerOptions::default()
					},
				}
				div {
					class: "rhs",
					div { 
						LightDark {}
					}
					match emoji() {
						Some(emoji) => {
							rsx! {
								div {
									class : "emoji",
									"{emoji.as_str()}"	
								}
							}
						},
						None => {
							rsx! {}
						}
					}
				}
			}
		}
	}
}
