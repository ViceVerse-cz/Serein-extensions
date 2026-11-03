use serein_extension_sdk::{export, Element, Invocation, Output};

pub const PALETTES: &[(&str, &[(&str, &str)])] = &[
	(
		"Approvals & Positivity",
		&[
			("thumbsup", "👍"),
			("heart", "❤️"),
			("tada", "🎉"),
			("fire", "🔥"),
			("rocket", "🚀"),
			("hundred", "💯"),
			("sparkles", "✨"),
			("clap", "👏"),
		],
	),
	(
		"Expressions & Humor",
		&[
			("joy", "😂"),
			("skull", "💀"),
			("moyai", "🗿"),
			("pleading", "🥺"),
			("thinking", "🤔"),
			("eyes", "👀"),
			("clown", "🤡"),
			("partying", "🥳"),
		],
	),
	(
		"Focus & Agreement",
		&[
			("check", "✅"),
			("target", "🎯"),
			("bulb", "💡"),
			("zap", "⚡"),
			("handshake", "🤝"),
			("brain", "🧠"),
			("pin", "📌"),
			("star", "🌟"),
		],
	),
];

fn emoji_for_id(id: &str) -> Option<&'static str> {
	for (_, list) in PALETTES {
		for &(key, emoji) in *list {
			if id == key {
				return Some(emoji);
			}
		}
	}
	None
}

pub fn handle(input: Invocation) -> Output {
	let action = input.action.as_str();
	let composer = input.composer.as_deref().unwrap_or("").trim();

	if let Some(emoji_key) = action.strip_prefix("add-") {
		if let Some(emoji) = emoji_for_id(emoji_key) {
			let new_text = if composer.is_empty() {
				emoji.to_string()
			} else {
				format!("{composer} {emoji}")
			};
			return Output {
				replacement: Some(new_text),
				panel: vec![
					Element::Heading {
						text: format!("Added Reaction: {emoji}"),
					},
					Element::Text {
						text: "Emoji appended to your active composer draft.".into(),
					},
				],
				..Default::default()
			};
		}
	}

	let mut panel = vec![
		Element::Heading {
			text: "Quick Reactions Palette".into(),
		},
		Element::Text {
			text: "Select a curated reaction to append directly to your message draft:".into(),
		},
		Element::Separator,
	];

	for (category, items) in PALETTES {
		panel.push(Element::Heading {
			text: category.to_string(),
		});

		let mut buttons = Vec::new();
		for &(key, emoji) in *items {
			buttons.push(Element::Button {
				id: format!("add-{key}"),
				label: format!("{emoji} :{key}:"),
			});
		}

		panel.push(Element::Row { children: buttons });
	}

	Output {
		panel,
		..Default::default()
	}
}

export!(handle);

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn appends_emoji_to_existing_draft() {
		let input = Invocation {
			action: "add-fire".into(),
			composer: Some("Awesome work!".into()),
			..Default::default()
		};
		let output = handle(input);
		assert_eq!(output.replacement.as_deref(), Some("Awesome work! 🔥"));
	}

	#[test]
	fn sets_emoji_to_empty_draft() {
		let input = Invocation {
			action: "add-rocket".into(),
			composer: None,
			..Default::default()
		};
		let output = handle(input);
		assert_eq!(output.replacement.as_deref(), Some("🚀"));
	}

	#[test]
	fn renders_all_palettes() {
		let input = Invocation {
			action: "picker".into(),
			..Default::default()
		};
		let output = handle(input);
		assert!(output.panel.len() >= 6);
	}
}
