use serein_extension_sdk::{export, Element, Invocation, Output};
use url::Url;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MessageLinkInfo {
	pub guild_id: String,
	pub channel_id: String,
	pub message_id: String,
	pub url: String,
}

pub fn parse_discord_message_link(input: &str) -> Option<MessageLinkInfo> {
	let trimmed = input.trim();
	let parsed = Url::parse(trimmed).ok()?;

	let host = parsed.host_str()?;
	if !host.ends_with("discord.com") && !host.ends_with("discordapp.com") {
		return None;
	}

	let segments: Vec<&str> = parsed
		.path_segments()?
		.filter(|s| !s.is_empty())
		.collect();

	// Format: /channels/{guild_id}/{channel_id}/{message_id}
	if segments.len() == 4 && segments[0] == "channels" {
		Some(MessageLinkInfo {
			guild_id: segments[1].to_string(),
			channel_id: segments[2].to_string(),
			message_id: segments[3].to_string(),
			url: trimmed.to_string(),
		})
	} else {
		None
	}
}

pub fn handle(input: Invocation) -> Output {
	let action = input.action.as_str();

	let text = input
		.selected_message
		.as_deref()
		.or(input.composer.as_deref())
		.unwrap_or("")
		.trim();

	if text.is_empty() {
		return Output {
			panel: vec![
				Element::Heading {
					text: "Message Link Embeds".into(),
				},
				Element::Text {
					text: "Select a message or enter a Discord message link (https://discord.com/channels/...) to format."
						.into(),
				},
			],
			..Default::default()
		};
	}

	if let Some(info) = parse_discord_message_link(text) {
		let formatted = format!("💬 [Jump to Message #{}](https://discord.com/channels/{}/{}/{})", info.message_id, info.guild_id, info.channel_id, info.message_id);
		let panel = vec![
			Element::Heading {
				text: "Discord Message Link Detected".into(),
			},
			Element::Text {
				text: format!("Guild: {} | Channel: {} | Message: {}", info.guild_id, info.channel_id, info.message_id),
			},
			Element::Text {
				text: format!("Preview: {formatted}"),
			},
			Element::Button {
				id: "format-link".into(),
				label: "Insert Jump Link to Composer".into(),
			},
		];

		if action == "format-link" {
			Output {
				replacement: Some(formatted),
				panel,
				..Default::default()
			}
		} else {
			Output {
				panel,
				..Default::default()
			}
		}
	} else {
		// Selected message quotation mode
		let quote_body = if text.len() > 100 {
			let prefix: String = text.chars().take(97).collect();
			format!("{prefix}…")
		} else {
			text.to_string()
		};

		let quoted_markdown = format!("> {}", quote_body.replace('\n', "\n> "));

		let panel = vec![
			Element::Heading {
				text: "Quote Selected Message".into(),
			},
			Element::Text {
				text: format!("Quote Preview:\n{quoted_markdown}"),
			},
			Element::Button {
				id: "quote-link".into(),
				label: "Insert Quote to Composer".into(),
			},
		];

		if action == "quote-link" {
			Output {
				replacement: Some(quoted_markdown),
				panel,
				..Default::default()
			}
		} else {
			Output {
				panel,
				..Default::default()
			}
		}
	}
}

export!(handle);

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn parses_discord_message_link() {
		let link = "https://discord.com/channels/112233/445566/778899";
		let info = parse_discord_message_link(link).expect("valid link");
		assert_eq!(info.guild_id, "112233");
		assert_eq!(info.channel_id, "445566");
		assert_eq!(info.message_id, "778899");
	}

	#[test]
	fn formats_discord_link_replacement() {
		let input = Invocation {
			action: "format-link".into(),
			composer: Some("https://discord.com/channels/111/222/333".into()),
			..Default::default()
		};
		let output = handle(input);
		assert!(output.replacement.is_some());
		assert!(output.replacement.unwrap().contains("Jump to Message #333"));
	}

	#[test]
	fn formats_quote_from_selected_message() {
		let input = Invocation {
			action: "quote-link".into(),
			selected_message: Some("Hello there!\nHow are you?".into()),
			..Default::default()
		};
		let output = handle(input);
		assert_eq!(
			output.replacement.as_deref(),
			Some("> Hello there!\n> How are you?")
		);
	}
}
