use dioxus::prelude::*;
use dioxus_emoji_picker::prelude::*;
use dioxus_emoji_picker::emoji_picker::options::*;

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
fn CubeIcon() -> Element {

	rsx! {
		svg {
			class: "cube-icon",
			width:"1em",
			height:"1em",
			view_box:"0 0 24 24",
			fill:"none",
			xmlns:"http://www.w3.org/2000/svg",
			g{
				id:"SVGRepo_bgCarrier",
				stroke_width:"0",
			}
			g {
				id:"SVGRepo_tracerCarrier",
				stroke_linecap:"round",
				stroke_linejoin:"round",
			}
			g {
				id:"SVGRepo_iconCarrier",
				path {
					d:"M12 10.2308L3.08495 7.02346M12 10.2308L20.9178 7.03406M12 10.2308V20.8791M5.13498 18.5771L10.935 20.6242C11.3297 20.7635 11.527 20.8331 11.7294 20.8608C11.909 20.8853 12.091 20.8853 12.2706 20.8608C12.473 20.8331 12.6703 20.7635 13.065 20.6242L18.865 18.5771C19.6337 18.3058 20.018 18.1702 20.3018 17.9269C20.5523 17.7121 20.7459 17.4386 20.8651 17.1308C21 16.7823 21 16.3747 21 15.5595V8.44058C21 7.62542 21 7.21785 20.8651 6.86935C20.7459 6.56155 20.5523 6.28804 20.3018 6.0732C20.018 5.82996 19.6337 5.69431 18.865 5.42301L13.065 3.37595C12.6703 3.23665 12.473 3.167 12.2706 3.13936C12.091 3.11484 11.909 3.11484 11.7294 3.13936C11.527 3.167 11.3297 3.23665 10.935 3.37595L5.13498 5.42301C4.36629 5.69431 3.98195 5.82996 3.69824 6.0732C3.44766 6.28804 3.25414 6.56155 3.13495 6.86935C3 7.21785 3 7.62542 3 8.44058V15.5595C3 16.3747 3 16.7823 3.13495 17.1308C3.25414 17.4386 3.44766 17.7121 3.69824 17.9269C3.98195 18.1702 4.36629 18.3058 5.13498 18.5771Z",
					stroke:"#000000",
					stroke_width:"2",
					stroke_linecap:"round",
					stroke_linejoin:"round",
				}
			}
		}
	}
}

#[component]
fn GitIcon() -> Element {
	rsx! {
		svg {
			width:"1em",
			height:"1em",
			view_box:"0 0 24 24",
			fill:"none",
			xmlns:"http://www.w3.org/2000/svg",
			g{
				id:"SVGRepo_bgCarrier",
				stroke_width:"0",
			}
			g { 
				id:"SVGRepo_tracerCarrier",
				stroke_linecap:"round",
				stroke_linejoin:"round",
			}
			g { 
				id:"SVGRepo_iconCarrier",
				path { 
					fill_rule:"evenodd",
					clip_rule:"evenodd",
					d:"M6 5C6 4.44772 6.44772 4 7 4C7.55228 4 8 4.44772 8 5C8 5.55228 7.55228 6 7 6C6.44772 6 6 5.55228 6 5ZM8 7.82929C9.16519 7.41746 10 6.30622 10 5C10 3.34315 8.65685 2 7 2C5.34315 2 4 3.34315 4 5C4 6.30622 4.83481 7.41746 6 7.82929V16.1707C4.83481 16.5825 4 17.6938 4 19C4 20.6569 5.34315 22 7 22C8.65685 22 10 20.6569 10 19C10 17.7334 9.21506 16.6501 8.10508 16.2101C8.45179 14.9365 9.61653 14 11 14H13C16.3137 14 19 11.3137 19 8V7.82929C20.1652 7.41746 21 6.30622 21 5C21 3.34315 19.6569 2 18 2C16.3431 2 15 3.34315 15 5C15 6.30622 15.8348 7.41746 17 7.82929V8C17 10.2091 15.2091 12 13 12H11C9.87439 12 8.83566 12.3719 8 12.9996V7.82929ZM18 6C18.5523 6 19 5.55228 19 5C19 4.44772 18.5523 4 18 4C17.4477 4 17 4.44772 17 5C17 5.55228 17.4477 6 18 6ZM6 19C6 18.4477 6.44772 18 7 18C7.55228 18 8 18.4477 8 19C8 19.5523 7.55228 20 7 20C6.44772 20 6 19.5523 6 19Z",
					fill:"#000000",
				}
			}
		}
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
				},
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
	let options = use_signal( EmojiPickerOptions::default );
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
			h3 {
				div { 
					a {
						href : "https://crates.io/crates/dioxus_emoji_picker",
						CubeIcon {},
						"crates.io"
					}
				}
				div {
					a {
						href : "https://github.com/hugohp/dioxus_emoji_picker",
						span {
							GitIcon {},
						}
						"repository"
					}
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
