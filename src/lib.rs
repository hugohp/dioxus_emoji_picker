#![doc = include_str!("../README.md")]
pub mod emoji_picker;

// Re-exports
pub mod prelude {
    pub use crate::emoji_picker::EmojiPicker;
    pub use crate::emoji_picker::options::EmojiPickerOptions;
}
