use emojis::{Emoji, SkinTone};
use std::cmp::*;

#[derive(Debug)]
pub struct EmojiIndexer {
    emojis: Vec<(String, &'static Emoji)>,
}

impl EmojiIndexer {
    pub fn new() -> Self {
        let mut emojis = Vec::new();

        for e in emojis::iter() {
            let name = e.name();
            let name = name.strip_prefix("flag: ").unwrap_or(name);
            emojis.push((name.to_lowercase(), e));

            if let Some(shortcode) = e.shortcode() {
                emojis.push((shortcode.to_lowercase(), e));
            }
        }

        emojis.sort_by(|lhs, rhs| lhs.0.cmp(&rhs.0));

        Self { emojis }
    }

    pub fn search(&self, what: &str, skin_tone: SkinTone) -> Vec<&'static Emoji> {
        if what.is_empty() {
            return vec![];
        }

        let what = what.to_lowercase();

        let lower_bound = self
            .emojis
            .partition_point(|(name, _)| name.as_str() < what.as_str());

        let mut results = vec![];

        for (name, emoji) in self.emojis.iter().skip(lower_bound) {
            if name.starts_with(&what) {
                let emoji = emoji.with_skin_tone(skin_tone).unwrap_or(emoji);

                if !results.contains(&emoji) {
                    results.push(emoji);
                }
            } else {
                break;
            }
        }
        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_empty() {
        let indexer = EmojiIndexer::new();
        let result = indexer.search("", SkinTone::Default);
        assert!(result.is_empty());
    }

    #[test]
    fn search_by_country() {
        let indexer = EmojiIndexer::new();
        let result = indexer.search("portugal", SkinTone::Default);
        assert_eq!(result, vec![emojis::get_by_shortcode("portugal").unwrap()]);
    }

    #[test]
    fn search_by_shortcode() {
        let indexer = EmojiIndexer::new();
        let result = indexer.search("rofl", SkinTone::Default);
        assert!(!result.is_empty());
        assert_eq!(result[0].shortcode(), Some("rofl"));
    }
}
