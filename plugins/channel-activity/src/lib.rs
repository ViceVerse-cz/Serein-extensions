use serde::{Deserialize, Serialize};
use serein_extension_sdk::{export, AppInvocation, Element, MessageEventKind, Output};
use std::collections::BTreeMap;

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActivityStorage {
	pub total_messages: u64,
	pub deleted_messages: u64,
	pub total_chars: u64,
	/// Author ID -> message count
	pub authors: BTreeMap<String, u64>,
}

pub fn handle(input: AppInvocation) -> Output {
	let action = input.invocation.action.as_str();
	match action {
		"on-message-event" => handle_message_event(input),
		"show-activity" => render_panel(&input, None),
		"reset-activity" => {
			let storage = ActivityStorage::default();
			let serialized = serde_json::to_string(&storage).ok();
			let mut output = render_panel(&input, Some(&storage));
			output.storage = serialized;
			output
		}
		_ => Output::default(),
	}
}

fn load_storage(input: &AppInvocation) -> ActivityStorage {
	input
		.invocation
		.storage_json::<ActivityStorage>()
		.ok()
		.flatten()
		.unwrap_or_default()
}

fn handle_message_event(input: AppInvocation) -> Output {
	let Some(event) = &input.message_event else {
		return Output::default();
	};

	let mut storage = load_storage(&input);

	match event.kind {
		MessageEventKind::Create => {
			storage.total_messages += 1;
			if let Some(content) = &event.content {
				storage.total_chars += content.chars().count() as u64;
			}
			if let Some(author) = &event.author_id {
				*storage.authors.entry(author.clone()).or_insert(0) += 1;
				if storage.authors.len() > 100 {
					let mut pairs: Vec<(String, u64)> = storage.authors.into_iter().collect();
					pairs.sort_by(|a, b| b.1.cmp(&a.1));
					pairs.truncate(100);
					storage.authors = pairs.into_iter().collect();
				}
			}
		}
		MessageEventKind::Delete => {
			storage.deleted_messages += 1;
		}
		MessageEventKind::Update => {}
	}

	Output {
		storage: serde_json::to_string(&storage).ok(),
		..Default::default()
	}
}

fn render_panel(input: &AppInvocation, explicit_storage: Option<&ActivityStorage>) -> Output {
	let fallback;
	let storage = match explicit_storage {
		Some(s) => s,
		None => {
			fallback = load_storage(input);
			&fallback
		}
	};

	let avg_len = if storage.total_messages > 0 {
		storage.total_chars / storage.total_messages
	} else {
		0
	};

	let mut panel = vec![
		Element::Heading {
			text: "Channel Activity Monitor".to_owned(),
		},
		Element::Text {
			text: "Анализ сообщений, скорости чата и активности участников в реальном времени."
				.to_owned(),
		},
		Element::Separator,
		Element::Row {
			children: vec![
				Element::Text {
					text: format!("Сообщений: {}", storage.total_messages),
				},
				Element::Text {
					text: format!("Удалено: {}", storage.deleted_messages),
				},
				Element::Text {
					text: format!("Ср. длина: {avg_len} симв."),
				},
			],
		},
		Element::Separator,
	];

	if storage.authors.is_empty() {
		panel.push(Element::Text {
			text: "Пока нет зафиксированных сообщений. Отправьте или дождитесь сообщений в канале."
				.to_owned(),
		});
	} else {
		panel.push(Element::Heading {
			text: "Топ участников по сообщениям:".to_owned(),
		});

		let mut authors_sorted: Vec<(&String, &u64)> = storage.authors.iter().collect();
		authors_sorted.sort_by(|a, b| b.1.cmp(a.1));

		let max_count = authors_sorted.first().map(|(_, cnt)| **cnt).unwrap_or(1).max(1);

		for (i, (author, count)) in authors_sorted.into_iter().take(5).enumerate() {
			let percent = (*count * 100) / max_count;
			let bar_filled = (percent / 10).min(10) as usize;
			let bar_empty = 10 - bar_filled;
			let bar = format!("{}{}", "█".repeat(bar_filled), "░".repeat(bar_empty));

			panel.push(Element::Text {
				text: format!("{}. Автор {} — {} сообщ. [{bar}] {percent}%", i + 1, author, count),
			});
		}
	}

	panel.push(Element::Separator);
	panel.push(Element::Button {
		id: "reset-activity".to_owned(),
		label: "Сбросить статистику".to_owned(),
	});

	Output {
		panel,
		..Default::default()
	}
}

export!(handle);

#[cfg(test)]
mod tests {
	use super::*;
	use serein_extension_sdk::{Invocation, MessageEvent};

	#[test]
	fn test_message_event_create_and_delete() {
		let input = AppInvocation {
			invocation: Invocation {
				action: "on-message-event".to_owned(),
				..Default::default()
			},
			message_event: Some(MessageEvent {
				kind: MessageEventKind::Create,
				channel_id: "ch1".to_owned(),
				message_id: "m1".to_owned(),
				author_id: Some("u100".to_owned()),
				content: Some("Hello, world!".to_owned()),
			}),
			..Default::default()
		};

		let output = handle(input);
		assert!(output.panel.is_empty(), "MessageEvent must not return panel");
		let storage_str = output.storage.clone().expect("must save storage");
		let storage: ActivityStorage = serde_json::from_str(&storage_str).unwrap();
		assert_eq!(storage.total_messages, 1);
		assert_eq!(storage.total_chars, 13);
		assert_eq!(storage.authors.get("u100"), Some(&1));

		// Now test delete event
		let del_input = AppInvocation {
			invocation: Invocation {
				action: "on-message-event".to_owned(),
				storage: output.storage,
				..Default::default()
			},
			message_event: Some(MessageEvent {
				kind: MessageEventKind::Delete,
				channel_id: "ch1".to_owned(),
				message_id: "m1".to_owned(),
				author_id: None,
				content: None,
			}),
			..Default::default()
		};

		let del_output = handle(del_input);
		let updated: ActivityStorage =
			serde_json::from_str(&del_output.storage.expect("must update storage")).unwrap();
		assert_eq!(updated.total_messages, 1);
		assert_eq!(updated.deleted_messages, 1);
	}

	#[test]
	fn test_panel_and_reset_activity() {
		let mut storage = ActivityStorage::default();
		storage.total_messages = 5;
		storage.total_chars = 100;
		storage.authors.insert("u101".to_owned(), 3);
		storage.authors.insert("u102".to_owned(), 2);

		let input = AppInvocation {
			invocation: Invocation {
				action: "show-activity".to_owned(),
				storage: Some(serde_json::to_string(&storage).unwrap()),
				..Default::default()
			},
			..Default::default()
		};

		let output = handle(input);
		assert!(!output.panel.is_empty());

		// Test reset
		let reset_input = AppInvocation {
			invocation: Invocation {
				action: "reset-activity".to_owned(),
				storage: Some(serde_json::to_string(&storage).unwrap()),
				..Default::default()
			},
			..Default::default()
		};

		let reset_output = handle(reset_input);
		let cleared: ActivityStorage =
			serde_json::from_str(&reset_output.storage.expect("must save cleared storage")).unwrap();
		assert_eq!(cleared.total_messages, 0);
		assert!(cleared.authors.is_empty());
	}
}
