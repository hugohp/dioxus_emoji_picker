use dioxus::prelude::*;
use emojis::{Group,SkinTone};
use lazy_static::lazy_static;

const SEARCH_ICON: Asset = asset!("/assets/emoji_picker_search_icon.png");
static SKIN_TONE: GlobalSignal<SkinTone> = Signal::global(|| SkinTone::Default);

mod emoji_indexer;

use emoji_indexer::*;

lazy_static! {
    static ref EMOJI_INDEXER: EmojiIndexer = EmojiIndexer::new();
}


#[derive(Clone,PartialEq)]
enum PickerStatus {
	ByGroup(Group),
	Searching(String),
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
		PickerStatus::ByGroup(_) => {
			rsx! {
				div {
					class: "emoji_groups",
					{
						groups.iter().map(|(group,group_emoji,group_name)| {
							let group = *group;
							rsx! {
								button {
									onclick: move |_| {
										picker_status
											.set(PickerStatus::ByGroup(group));
									},
									key : "{group_emoji}",
									class : "emoji_button",
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
fn EmojiSearch(
	picker_status : Signal<PickerStatus>,
) -> Element {

	let mut show_skin_tones = use_signal(|| false);
	let skin_tone_icon = use_memo(|| {
		let current_skin_tone = SKIN_TONE.read().clone();
		emojis::get("✌️").unwrap().with_skin_tone(current_skin_tone).unwrap().as_str()
	});

	rsx! {
		div {
			class: "emoji_search",
			section {
				class: "emoji_search_box",
				input {
					class: "emoji_search_box_input",
					oninput: move |evt| {
						let what = evt.value().clone();
						if what.is_empty() {
							picker_status
								.set(PickerStatus::ByGroup(Group::SmileysAndEmotion));
						} else {
							picker_status.set(PickerStatus::Searching(what));
						}
					},
					type: "text",
					placeholder: "Search...",
				}
				div {
					class: "emoji_search_icon",
					img { src : "{SEARCH_ICON}" }
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
fn EmojiGrid(
	picker_status : Signal<PickerStatus>,
	content : Signal<String>,
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
										content.with_mut(|c|
											c.push_str(emoji_str)
										);
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
										content.with_mut(|c|
											c.push_str(emoji_str)
										);
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
	content : Signal<String>
) -> Element {
	
	let picker_status = use_signal(|| 
		PickerStatus::ByGroup(Group::SmileysAndEmotion)
	);

	rsx! {
		div {
			class: "emoji_container",
			EmojiGroups { picker_status : picker_status },
			EmojiSearch { picker_status : picker_status },
			EmojiGrid { picker_status : picker_status , content : content},
		}
	}
}
