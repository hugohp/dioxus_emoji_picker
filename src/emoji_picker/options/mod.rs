use emojis::Emoji;

#[derive(Clone,PartialEq)]
pub struct EmojiPickerOptions {
	pub skin_tone_emoji : &'static Emoji, 
}

impl EmojiPickerOptions {

	pub fn new() -> Self {
		Self::default()
	}

	pub fn with_skin_tone_emoji(
		self,
		emoji : &'static Emoji
	) -> Self {
		Self { 
			skin_tone_emoji : emoji 
		}
	}
}

impl Default for EmojiPickerOptions {
	fn default() -> EmojiPickerOptions {
		Self { 	
			skin_tone_emoji : emojis::get("✌️").unwrap(),
		}
	}
}
