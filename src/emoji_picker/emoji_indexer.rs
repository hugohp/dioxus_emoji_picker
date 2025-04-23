use emojis::{Emoji,SkinTone};
use std::cmp::*;

#[derive(Debug)]
pub struct EmojiIndexer {
    emojis : Vec<(String,&'static Emoji)>,
}

impl EmojiIndexer {
    pub fn new() -> Self {

		let mut emojis = emojis::iter().map(|e|
			(e.name().to_lowercase(),e)
		)
		.collect::<Vec<(String,&'static Emoji)>>();

		emojis.sort_by(|lhs,rhs| lhs.0.cmp(&rhs.0));

		Self { emojis : emojis }
    }

	pub fn search(&self, what: &str, skin_tone: SkinTone) -> Vec<&'static Emoji> {
			
		let what = what.to_lowercase();

		let mut lower_bound = self.emojis.as_slice()
			.binary_search_by(|(name,_)| 
				match name.cmp(&what) {
					Ordering::Equal => Ordering::Greater,
					ord => ord
				}
			)
			.unwrap_err();

		let mut results = vec![];

		while lower_bound < self.emojis.len() {
			if let Some((name,emoji)) = self.emojis.get(lower_bound) {
				if name.as_str().starts_with(&what) {
					let emoji = 
						emoji.with_skin_tone(skin_tone).unwrap_or(emoji);
					results.push(emoji);
				} else {
					break;
				}
			}
			lower_bound += 1;
		}
		results
	}
}
