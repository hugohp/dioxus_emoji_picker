use emojis::Emoji;

#[derive(Clone,PartialEq,Default)]
pub enum Theme {
	#[default]
	Auto, 
	Light, 
	Dark
}


#[derive(Clone,PartialEq)]
pub struct EmojiPickerOptions {
	pub skin_tone_emoji : &'static Emoji, 
	pub theme : Theme,
}

impl EmojiPickerOptions {

	pub fn new() -> Self {
		Self::default()
	}

	pub fn with_skin_tone_emoji(
		mut self,
		emoji : &'static Emoji,
	) -> Self {
		self.skin_tone_emoji = emoji;
		self
	}

	pub fn with_theme(
		mut self,
		theme : Theme
	) -> Self {
		self.theme = theme;
		self
	}

}

impl Default for EmojiPickerOptions {
	fn default() -> EmojiPickerOptions {
		Self { 	
			skin_tone_emoji : emojis::get("✌️").unwrap(),
			theme : Theme::default()
		}
	}
}
