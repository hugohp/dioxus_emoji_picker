use dioxus::prelude::*;
use dioxus_emoji_picker::prelude::*;
use dioxus_emoji_picker::emoji_picker::options::*;
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
fn LightDark(
	options : Signal<EmojiPickerOptions>,
) -> Element {

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
				onchange: move |_| {
					options.set(
						options().with_theme(Theme::default())
					)
				}
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
				onchange: move |_| {
					options.set(
						options().with_theme(Theme::Dark)
					)
				}
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
				onchange: move |_| {
					options.set(
						options().with_theme(Theme::Light)
					)
				}
		
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
	let emoji = use_signal(|| None);
	let options = use_signal( || EmojiPickerOptions::default() );
	rsx! {
		div {
			class : "top",
			h1 {
				"dioxus_emoji_picker"
			}
			h2 {
				"Emoji Picker for "
				a {
					href : "https://dioxuslabs.com/",
					"Dioxus Web Framework"
				}
			}
			h2 {
				a {
					href : "https://crates.io/crates/dioxus_emoji_picker",
					"crates.io"
				}
			}
			div {
				class: "demo",
				div {
					class: "lhs",
					EmojiPicker { 
						emoji : emoji,
						options : options,
					},
				}
				div {
					class: "rhs",
					div { 
						LightDark { options : options }
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
