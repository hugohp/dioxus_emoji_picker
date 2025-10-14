use dioxus::prelude::*;
use emojis::{Emoji,Group,SkinTone};
use lazy_static::lazy_static;

static SKIN_TONE: GlobalSignal<SkinTone> = Signal::global(|| SkinTone::Default);

pub mod options;
mod emoji_indexer;

use emoji_indexer::*;
use options::*;

lazy_static! {
    static ref EMOJI_INDEXER: EmojiIndexer = EmojiIndexer::new();
}

#[component]
fn SearchIcon() -> Element {

	// <!-- Uploaded to: SVG Repo, www.svgrepo.com, Generator: SVG Repo Mixer Tools -->
	rsx! {
		svg {
			class: "search-icon",
			view_box:"0 0 24 24",
			xmlns:"http://www.w3.org/2000/svg",
			path {
				d:"M15.7955 15.8111L21 21M18 10.5C18 14.6421 14.6421 18 10.5 18C6.35786 18 3 14.6421 3 10.5C3 6.35786 6.35786 3 10.5 3C14.6421 3 18 6.35786 18 10.5Z",
				stroke_width:"2",
				stroke_linecap:"round",
				stroke_linejoin:"round",
			}
		}
	}
}

#[component]
fn CancelIcon(
	picker_status : Signal<PickerStatus>,
	search_text : Signal<String>,
) -> Element {

	// <!-- Uploaded to: SVG Repo, www.svgrepo.com, Generator: SVG Repo Mixer Tools -->

	rsx! {
		span {
			onclick: move |_| {
				picker_status.set(
					PickerStatus::ByGroup(Group::SmileysAndEmotion)
				);
				search_text.set(String::new());
			},
			svg {
				class: "cancel-icon",
				view_box:"0 0 32 32",
				xmlns:"http://www.w3.org/2000/svg",
				version:"1.1" ,
				path {
					d:"M19.587 16.001l6.096 6.096c0.396 0.396 0.396 1.039 0 1.435l-2.151 2.151c-0.396 0.396-1.038 0.396-1.435 0l-6.097-6.096-6.097 6.096c-0.396 0.396-1.038 0.396-1.434 0l-2.152-2.151c-0.396-0.396-0.396-1.038 0-1.435l6.097-6.096-6.097-6.097c-0.396-0.396-0.396-1.039 0-1.435l2.153-2.151c0.396-0.396 1.038-0.396 1.434 0l6.096 6.097 6.097-6.097c0.396-0.396 1.038-0.396 1.435 0l2.151 2.152c0.396 0.396 0.396 1.038 0 1.435l-6.096 6.096z"
				}
			}
		}
	}
}

fn group_as_str(group : Group) -> &'static str {

	match group {
		Group::SmileysAndEmotion => {
			"Smileys & Emotion"
		},
		Group::PeopleAndBody => {
			"People & Body"
		},
		Group::AnimalsAndNature => {
			"Animals & Nature"
		},
		Group::FoodAndDrink => {
			"Food & Drink"
		},
		Group::TravelAndPlaces => {
			"Travel & Places"
		},
		Group::Activities => {
			"Activities"
		},
		Group::Objects => {
			"Objects"
		},
		Group::Symbols => {
			"Symbols"
		},
		Group::Flags => {
			"Flags"
		},
	}
}

#[derive(Clone,PartialEq)]
enum PickerStatus {
	ByGroup(Group),
	Searching(String),
}

#[component]
fn EmojiSearch(
	picker_status : Signal<PickerStatus>,
	options : Signal<EmojiPickerOptions>,
) -> Element {

	let mut show_skin_tones = use_signal(|| false);
	let mut search_text = use_signal(|| String::new());

	let skin_tone_emoji = use_memo(move || 
		options().skin_tone_emoji
	);

	let skin_tone_icon = use_memo(move || {
		let current_skin_tone = SKIN_TONE.read().clone();
		skin_tone_emoji().with_skin_tone(current_skin_tone).unwrap().as_str()
	});

	rsx! {
		div {
			class: "emoji_search",
			section {
				class: "emoji_search_box",
				input {
					class: "emoji_search_box_input",
					id: "emoji_search_box_input",
					oninput: move |evt| {
						let what = evt.value().clone();
						if what.is_empty() {
							picker_status
								.set(PickerStatus::ByGroup(Group::SmileysAndEmotion));
						} else {
							picker_status.set(PickerStatus::Searching(what.clone()));
						}
						search_text.set(what);
					},
					type: "text",
					placeholder: "Search...",
					value: "{search_text}"
				}
				span {
					class: "emoji_search_icon",
					match *picker_status.read() {
						PickerStatus::ByGroup(_) => {
							rsx! {
								SearchIcon {}
							}
						},
						PickerStatus::Searching(_) => {
							rsx! {
								CancelIcon { 
									picker_status : picker_status,
									search_text : search_text
								}
							}
						}
					}
				}
			}
			div {
				class: "emoji_skin_tone_dropdown",
				button {
					class : "emoji_button",
					onclick: move |_| {
						show_skin_tones.toggle();
					},
					"{skin_tone_icon}",
				}
				if show_skin_tones() {
					div {
						class: "emoji_skin_tone_list",
						{
							emojis::get(skin_tone_icon.read().clone()).unwrap()
								.skin_tones().unwrap()
								.map(|e| {
									let emoji_str = e.as_str();
									let shortcode = e.shortcode().unwrap_or_default();
									rsx! {
										button {
											onclick: move |_| {
												SKIN_TONE.with_mut(|s|
													*s = e.skin_tone().unwrap()
												);
												show_skin_tones.toggle();
											},
											class : "emoji_button",
											title : "{shortcode}",
											"{emoji_str}"
										}
									}
								})
						}
					}
				}
			}
		}
	}
}

#[component]
fn EmojiCategory(
	picker_status : Signal<PickerStatus>
) -> Element {

	match &*picker_status.read() {
		PickerStatus::ByGroup(group) => {
			let category = group_as_str(*group);
			rsx! {
				h2 {
					class : "emoji_category",
					"{category}"
				}
			}
		},
		_ => {
			rsx! {}
		}
	}
}

#[component]
fn EmojiGroups(
	picker_status : Signal<PickerStatus>
) -> Element {

    let groups = vec![
        (Group::SmileysAndEmotion, "😀","Smiley and Emotion"),
        (Group::PeopleAndBody, "👋","People and Body"),
        (Group::AnimalsAndNature, "🐶","Animals and Nature"),
        (Group::FoodAndDrink, "🍕","Foor and Drink"),
        (Group::TravelAndPlaces, "✈️","Travel and Places"),
        (Group::Activities, "⚽","Activities"),
        (Group::Objects, "💡","Objects"),
        (Group::Symbols, "❤️","Symbols"),
        (Group::Flags, "🏁","Flags"),
    ];

	match &*picker_status.read() {
		PickerStatus::ByGroup(selected_group) => {
			rsx! {
				div {
					class: "emoji_groups",
					{
						groups.iter().map(|(group,group_emoji,group_name)| {
							let group = *group;
							let class_name = if group == *selected_group {
								"emoji_groups_button active"
							} else {
								"emoji_groups_button"
							};
							rsx! {
								button {
									onclick: move |_| {
										picker_status
											.set(PickerStatus::ByGroup(group));
									},
									key : "{group_emoji}",
									class : "{class_name}",
									title : "{group_name}",
									"{group_emoji}"
								}
							}
						})
					}
				}
			}
		},
		PickerStatus::Searching(_) => {
			rsx! {}
		}
	}
}		


#[component]
fn EmojiGrid(
	picker_status : Signal<PickerStatus>,
	emoji : Signal<Option<&'static Emoji>>,
) -> Element {

	match &*picker_status.read() {
		PickerStatus::ByGroup(group) => {
			rsx! {
				div {
					class: "emoji_grid",
					{
						group.emojis().map(|e| {
							let current_skin_tone = *SKIN_TONE.read();
							let e1 = e.with_skin_tone(current_skin_tone)
								.unwrap_or(e);

							let emoji_str = e1.as_str();
							let shortcode = e1.shortcode().unwrap_or_default();
							rsx! {
								button {
									onclick: move |_| {
										emoji.set(Some(e1));
									},
									key : "{emoji_str}",
									class : "emoji_button",
									title : "{shortcode}",
									"{emoji_str}"
								}
							}
						})
					}
				}
			}
		},
		PickerStatus::Searching(what) => {
			let results = EMOJI_INDEXER.search(what, *SKIN_TONE.read());
			rsx! {
				div {
					class: "emoji_grid",
					{
						results.iter().map(|e| {
							let current_skin_tone = *SKIN_TONE.read();
							let e1 = e.with_skin_tone(current_skin_tone)
								.unwrap_or(e);

							let emoji_str = e1.as_str();
							let shortcode = e1.shortcode().unwrap_or_default();
							rsx! {
								button {
									onclick: move |_| {
										emoji.set(Some(e1));
									},
									key : "{emoji_str}",
									class : "emoji_button",
									title : "{shortcode}",
									"{emoji_str}"
								}
							}
						})
					}
				}
			}
		}
	}
}


#[component]
pub fn EmojiPicker(
	emoji : Signal<Option<&'static Emoji>>,
	options : Signal<EmojiPickerOptions>
) -> Element {
	
	let picker_status = use_signal(||
		PickerStatus::ByGroup(Group::SmileysAndEmotion)
	);

	let data_theme = use_memo(move ||
		match options().theme { 
			Theme::Auto => "",
			Theme::Light => "light",
			Theme::Dark => "dark",
		}
	);

	rsx! {
		emoji-picker {
			class: "emoji_picker",
			"data-theme": "{data_theme}",
			EmojiSearch { 
				picker_status : picker_status , 
				options : options 
			},
			EmojiGroups { picker_status : picker_status },
			EmojiCategory { picker_status : picker_status },
			EmojiGrid { picker_status : picker_status , emoji : emoji},
		}
	}
}
