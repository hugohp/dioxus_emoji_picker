# dioxus-emoji-picker

<div align="center">
  <img src="images/emoji_picker.png">
</div>

# Usage

```rust

use dioxus_emoji_picker::prelude::*;

#[component]
fn example() -> Element
use dioxus_emoji_picker::prelude::*;

	let mut content = use_signal(|| String::new());

	rsx! {
		div {
			EmojiPicker { content : content },
		}
	}
}
```
