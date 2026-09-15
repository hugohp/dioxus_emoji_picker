# Dioxus Emoji Picker

<div align="center">
  <img src="https://github.com/hugohp/dioxus_emoji_picker/blob/main/images/emoji_picker.png?raw=true" alt="Emoji Picker">
</div>

An emoji picker for the [Dioxus](https://dioxuslabs.com/) web framework.

# Usage

```rust
use dioxus::prelude::*;
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

```css
emoji-picker {
  --emoji-size: 3rem;
  --num-columns: 6;
  --background: gray;
}
```

## Light and dark mode

Dark and light mode can be styled appending .light or .dark to emoji-picker 

```css
emoji-picker.dark {
  --border-color: #e0e0e0;
}
```

Full list of options:

| Variable | Default (light) | Default (dark) | Description |
|:-----|:--------:|:--------:|------:|
| \-\-font-size   | 1.5em | 1.5em | Font-size |
| \-\-emoji-size | 1.5em | 1.5em | Size of emojis |
| \-\-num-columns | 9 | 9 | Number of columns in emoji grid |
| \-\-num-rows | 10 | 10 | Number of rows in emoji grid |
| \-\-background | #fff | #222 | Background color |
| \-\-border-color | #e0e0e0 | #444 | Border color |
| \-\-category-color | #111 | #efefef | Font color for category |
| \-\-input-font-color | #111 | #efefef | Input font color |
| \-\-button-hover-background | #d9d9d9 | #555555 | Background of a hovered button |
| \-\-border-size | 1px | 1px | Width of border |
| \-\-border-radius | 0 | 0 | Radius of border |
| \-\-box-shadow | none | none | Box shadow |
| \-\-indicator-height | 3px | 3px | Indicator height |
| \-\-indicator-color | #385ac1 | #385ac1 | Indicator colour |
