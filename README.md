# Dioxus Emoji Picker

<div align="center">
  <img src="https://raw.githubusercontent.com/hugohp/dioxus_emoji_picker/205d3309102e756c2f9805b2e440968284e74d5c/images/emoji_picker.png" alt="Emoji Picker">
</div>

An emoji picker for the [Dioxus](https://dioxuslabs.com/) web framework.

# Usage

```rust
use dioxus_emoji_picker::prelude::*;
use dioxus_emoji_picker::emoji_picker::options::*;

#[component]
fn example() -> Element {

	let emoji = use_signal(|| None);
	let options = use_signal( || EmojiPickerOptions::default() );

	rsx! {
		div {
			EmojiPicker { 
				emoji : emoji,
				options : options,
			}
		}
	}
}
```

# Demo

Click [here](https://hugohp.github.io/dioxus_emoji_picker_demo/) for demo.

# CSS

Colors and size can be styled with CSS variables. For example:

```
emoji-picker {
  --emoji-size: 3rem;
  --num-columns: 6;
  --background: gray;
}
```

Full list of options:

| Variable | Default | Description |
|:-----|:--------:|------:|
| \-\-font-size   | 1.5em | Font-size |
| \-\-emoji-size | 1.5em | Size of emojis |
| \-\-num-columns | 9 | Number of columns in emoji grid |
| \-\-num-rows | 10 | Number of rows in emoji grid |
| \-\-background | #fff | Background color |
| \-\-background-dark | #222 | Background color (dark)|
| \-\-border-color | #e0e0e0 | Border color |
| \-\-border-color-dark | #444 | Border color (dark) |
| \-\-category-color | #111 | Font color for category |
| \-\-category-color-dark | #efefef | Font color for category (dark) |
| \-\-input-font-color | #111 | Input font color |
| \-\-input-font-color-dark | #efefef | Input font color (dark) |
| \-\-border-size | 1px | Width of border |
| \-\-border-radius | 0 | Radius of border |
| \-\-box-shadow | none | Box shadow |
