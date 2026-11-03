use serein_extension_sdk::{export, Element, Invocation, Output};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct CustomCommand {
	pub name: String,
	pub description: String,
	pub template: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct CommandsStore {
	pub commands: Vec<CustomCommand>,
}

impl Default for CommandsStore {
	fn default() -> Self {
		Self {
			commands: vec![
				CustomCommand {
					name: "shrug".into(),
					description: "Append shrug kaomoji".into(),
					template: r"¯\_(ツ)_/¯".into(),
				},
				CustomCommand {
					name: "tableflip".into(),
					description: "Flip the table in frustration".into(),
					template: "(╯°□°)╯︵ ┻━┻".into(),
				},
				CustomCommand {
					name: "unflip".into(),
					description: "Respectfully restore the table".into(),
					template: "┬─┬ノ( º _ ºノ)".into(),
				},
				CustomCommand {
					name: "lenny".into(),
					description: "Smug lenny face".into(),
					template: "( ͡° ͜ʖ ͡°)".into(),
				},
				CustomCommand {
					name: "afk".into(),
					description: "Set away message with optional reason".into(),
					template: "[AFK] Away from keyboard: {args}".into(),
				},
				CustomCommand {
					name: "rule".into(),
					description: "Prompt user to check rules".into(),
					template: "Please remember to adhere to the community guidelines and rules.".into(),
				},
			],
		}
	}
}

pub fn parse_store(storage: Option<&str>) -> CommandsStore {
	storage
		.and_then(|s| serde_json::from_str::<CommandsStore>(s).ok())
		.unwrap_or_default()
}

pub fn expand_command(text: &str, store: &CommandsStore) -> Option<String> {
	let trimmed = text.trim();
	if !trimmed.starts_with('/') {
		return None;
	}

	let line = &trimmed[1..];
	let mut parts = line.splitn(2, ' ');
	let cmd_name = parts.next()?.to_lowercase();
	let args = parts.next().unwrap_or("").trim();

	let matched = store.commands.iter().find(|c| c.name.to_lowercase() == cmd_name)?;

	let mut result = matched.template.clone();
	if result.contains("{args}") {
		result = result.replace("{args}", if args.is_empty() { "none" } else { args });
	} else if !args.is_empty() {
		result = format!("{result} {args}");
	}

	Some(result)
}

pub fn handle(input: Invocation) -> Output {
	let mut store = parse_store(input.storage.as_deref());
	let action = input.action.as_str();

	// Action 1: expand command from composer
	if action == "expand-command" {
		let composer_text = input.composer.as_deref().unwrap_or_default();
		if let Some(expanded) = expand_command(composer_text, &store) {
			return Output {
				replacement: Some(expanded),
				panel: vec![
					Element::Heading {
						text: "Command Expanded".into(),
					},
					Element::Text {
						text: "Custom command macro applied to message.".into(),
					},
				],
				..Default::default()
			};
		}
	}

	// Action 2: Add a new command
	if action == "add-command" {
		let name = input
			.values
			.get("command-name")
			.map(|s| s.trim().trim_start_matches('/').to_lowercase())
			.unwrap_or_default();
		let desc = input
			.values
			.get("command-desc")
			.map(|s| s.trim().to_owned())
			.unwrap_or_default();
		let template = input
			.values
			.get("command-template")
			.map(|s| s.trim().to_owned())
			.unwrap_or_default();

		if !name.is_empty() && !template.is_empty() {
			if let Some(pos) = store.commands.iter().position(|c| c.name == name) {
				store.commands[pos].template = template;
				store.commands[pos].description = desc;
			} else if store.commands.len() < 100 {
				store.commands.push(CustomCommand {
					name,
					description: desc,
					template,
				});
			}
		}
	}

	// Action 3: Delete command
	if let Some(stripped) = action.strip_prefix("delete-") {
		store.commands.retain(|c| c.name != stripped);
	}

	// Build settings / management panel
	let mut panel = vec![
		Element::Heading {
			text: "Custom Commands & Macros".into(),
		},
		Element::Text {
			text: format!("Registered custom commands: {}", store.commands.len()),
		},
		Element::Separator,
		Element::Heading {
			text: "Create New Command".into(),
		},
		Element::TextInput {
			id: "command-name".into(),
			label: "Command name (e.g. hello, afk, note):".into(),
			value: String::new(),
		},
		Element::TextInput {
			id: "command-desc".into(),
			label: "Description:".into(),
			value: String::new(),
		},
		Element::TextInput {
			id: "command-template".into(),
			label: "Template (use {args} for parameters):".into(),
			value: String::new(),
		},
		Element::Button {
			id: "add-command".into(),
			label: "Save Command".into(),
		},
		Element::Separator,
		Element::Heading {
			text: "Existing Commands".into(),
		},
	];

	for cmd in &store.commands {
		panel.push(Element::Text {
			text: format!("• **/{}:** {} (Template: `{}`)", cmd.name, cmd.description, cmd.template),
		});
		let del_id = format!("delete-{}", cmd.name.to_lowercase().replace('_', "-"));
		panel.push(Element::Button {
			id: del_id,
			label: format!("Delete /{}", cmd.name),
		});
	}

	let serialized_storage = serde_json::to_string(&store).ok();

	Output {
		panel,
		storage: serialized_storage,
		..Default::default()
	}
}

export!(handle);

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn parses_default_commands() {
		let store = parse_store(None);
		assert_eq!(store.commands.len(), 6);
		assert!(store.commands.iter().any(|c| c.name == "shrug"));
	}

	#[test]
	fn expands_simple_macro() {
		let store = parse_store(None);
		let res = expand_command("/shrug", &store);
		assert_eq!(res.as_deref(), Some(r"¯\_(ツ)_/¯"));
	}

	#[test]
	fn expands_macro_with_arguments() {
		let store = parse_store(None);
		let res = expand_command("/afk lunch break", &store);
		assert_eq!(res.as_deref(), Some("[AFK] Away from keyboard: lunch break"));
	}

	#[test]
	fn adds_and_deletes_custom_command() {
		let mut values = std::collections::BTreeMap::new();
		values.insert("command-name".into(), "wave".into());
		values.insert("command-desc".into(), "Wave hello".into());
		values.insert("command-template".into(), "Hey everyone! 👋".into());

		let input = Invocation {
			action: "add-command".into(),
			values,
			..Default::default()
		};

		let output = handle(input);
		assert!(output.storage.is_some());
		let store: CommandsStore = serde_json::from_str(&output.storage.unwrap()).unwrap();
		assert!(store.commands.iter().any(|c| c.name == "wave"));

		let delete_input = Invocation {
			action: "delete-wave".into(),
			storage: serde_json::to_string(&store).ok(),
			..Default::default()
		};

		let del_output = handle(delete_input);
		let del_store: CommandsStore = serde_json::from_str(&del_output.storage.unwrap()).unwrap();
		assert!(!del_store.commands.iter().any(|c| c.name == "wave"));
	}
}
