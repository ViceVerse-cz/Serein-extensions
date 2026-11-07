use serde::{Deserialize, Serialize};
use serein_extension_sdk::{export, AppInvocation, Element, Output, RelationshipKind};
use std::collections::BTreeMap;

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationshipStorage {
	/// Map of user_id -> display_name
	pub known_friends: BTreeMap<String, String>,
	/// Bounded event log (up to 20 recent events)
	pub history: Vec<RelationshipLogEntry>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationshipLogEntry {
	pub user_id: String,
	pub user_name: String,
	pub change_type: String,
	pub description: String,
}

pub fn handle(input: AppInvocation) -> Output {
	let action = input.invocation.action.as_str();
	match action {
		"on-relationship-event" => handle_event(input),
		"show-history" => render_panel(&input, None),
		"clear-history" => {
			let mut storage = load_storage(&input);
			storage.history.clear();
			let serialized = serde_json::to_string(&storage).ok();
			let mut output = render_panel(&input, Some(&storage));
			output.storage = serialized;
			output
		}
		_ => Output::default(),
	}
}

fn load_storage(input: &AppInvocation) -> RelationshipStorage {
	input
		.invocation
		.storage_json::<RelationshipStorage>()
		.ok()
		.flatten()
		.unwrap_or_default()
}

fn handle_event(input: AppInvocation) -> Output {
	let Some(app) = &input.app else {
		return Output::default();
	};
	let Some(relationships) = &app.relationships else {
		return Output::default();
	};

	let mut storage = load_storage(&input);
	let current_friends: BTreeMap<String, String> = relationships
		.items
		.iter()
		.filter(|item| item.kind == RelationshipKind::Friend)
		.map(|item| (item.user.id.clone(), item.user.name.clone()))
		.collect();

	// First run: initialize baseline without reporting false-positive removals
	if storage.known_friends.is_empty() {
		storage.known_friends = current_friends;
		return Output {
			storage: serde_json::to_string(&storage).ok(),
			..Default::default()
		};
	}

	// Detect removed friends
	for (old_id, old_name) in &storage.known_friends {
		if !current_friends.contains_key(old_id) {
			storage.history.push(RelationshipLogEntry {
				user_id: old_id.clone(),
				user_name: old_name.clone(),
				change_type: "Удален из друзей".to_owned(),
				description: format!("Пользователь {old_name} (ID: {old_id}) больше не в вашем списке друзей"),
			});
		}
	}

	// Detect newly added friends
	for (new_id, new_name) in &current_friends {
		if !storage.known_friends.contains_key(new_id) {
			storage.history.push(RelationshipLogEntry {
				user_id: new_id.clone(),
				user_name: new_name.clone(),
				change_type: "Добавлен в друзья".to_owned(),
				description: format!("Пользователь {new_name} (ID: {new_id}) добавлен в список друзей"),
			});
		}
	}

	// Bound history to last 20 entries
	if storage.history.len() > 20 {
		let remove_count = storage.history.len() - 20;
		storage.history.drain(0..remove_count);
	}

	storage.known_friends = current_friends;

	Output {
		storage: serde_json::to_string(&storage).ok(),
		..Default::default()
	}
}

fn render_panel(input: &AppInvocation, explicit_storage: Option<&RelationshipStorage>) -> Output {
	let fallback_storage;
	let storage = match explicit_storage {
		Some(s) => s,
		None => {
			fallback_storage = load_storage(input);
			&fallback_storage
		}
	};

	let friend_count = input
		.app
		.as_ref()
		.and_then(|a| a.relationships.as_ref())
		.map(|r| r.items.iter().filter(|i| i.kind == RelationshipKind::Friend).count())
		.unwrap_or(storage.known_friends.len());

	let mut panel = vec![
		Element::Heading {
			text: "Relationship Notifier".to_owned(),
		},
		Element::Text {
			text: "Автоматический мониторинг изменений списка друзей в Discord.".to_owned(),
		},
		Element::Separator,
		Element::Row {
			children: vec![
				Element::Text {
					text: format!("Текущих друзей: {friend_count}"),
				},
				Element::Text {
					text: format!("Зафиксировано событий: {}", storage.history.len()),
				},
			],
		},
		Element::Separator,
	];

	if storage.history.is_empty() {
		panel.push(Element::Text {
			text: "История пуста. События добавления и удаления друзей появятся здесь автоматически."
				.to_owned(),
		});
	} else {
		panel.push(Element::Heading {
			text: "История изменений:".to_owned(),
		});
		for entry in storage.history.iter().rev() {
			panel.push(Element::Text {
				text: format!("• [{}] {}", entry.change_type, entry.description),
			});
		}
	}

	panel.push(Element::Separator);
	panel.push(Element::Button {
		id: "clear-history".to_owned(),
		label: "Очистить историю".to_owned(),
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
	use serein_extension_sdk::{
		AppEventKind, AppSnapshot, Invocation, RelationshipSnapshot, RelationshipsSnapshot,
		UserSnapshot,
	};

	fn make_snapshot(friends: &[(&str, &str)]) -> RelationshipsSnapshot {
		RelationshipsSnapshot {
			items: friends
				.iter()
				.map(|&(id, name)| RelationshipSnapshot {
					user: UserSnapshot {
						id: id.to_owned(),
						name: name.to_owned(),
					},
					kind: RelationshipKind::Friend,
				})
				.collect(),
			truncated: false,
			friends_known: true,
			requests_known: true,
			restricted_known: true,
		}
	}

	#[test]
	fn test_baseline_population() {
		let snapshot = make_snapshot(&[("101", "Alice"), ("102", "Bob")]);
		let input = AppInvocation {
			invocation: Invocation {
				action: "on-relationship-event".to_owned(),
				..Default::default()
			},
			app: Some(AppSnapshot {
				relationships: Some(snapshot),
				..Default::default()
			}),
			app_event: Some(AppEventKind::Relationships),
			..Default::default()
		};

		let output = handle(input);
		assert!(output.panel.is_empty(), "AppEvent must never return panel");
		let storage: RelationshipStorage =
			serde_json::from_str(&output.storage.expect("must save storage")).unwrap();
		assert_eq!(storage.known_friends.len(), 2);
		assert!(storage.history.is_empty(), "Initial sync produces no false events");
	}

	#[test]
	fn test_friend_removal_detection() {
		let mut storage = RelationshipStorage::default();
		storage.known_friends.insert("101".to_owned(), "Alice".to_owned());
		storage.known_friends.insert("102".to_owned(), "Bob".to_owned());

		// Alice removed, only Bob remains
		let snapshot = make_snapshot(&[("102", "Bob")]);
		let input = AppInvocation {
			invocation: Invocation {
				action: "on-relationship-event".to_owned(),
				storage: Some(serde_json::to_string(&storage).unwrap()),
				..Default::default()
			},
			app: Some(AppSnapshot {
				relationships: Some(snapshot),
				..Default::default()
			}),
			app_event: Some(AppEventKind::Relationships),
			..Default::default()
		};

		let output = handle(input);
		let updated: RelationshipStorage =
			serde_json::from_str(&output.storage.expect("must update storage")).unwrap();
		assert_eq!(updated.known_friends.len(), 1);
		assert_eq!(updated.history.len(), 1);
		assert_eq!(updated.history[0].user_name, "Alice");
		assert_eq!(updated.history[0].change_type, "Удален из друзей");
	}

	#[test]
	fn test_panel_and_clear_history() {
		let mut storage = RelationshipStorage::default();
		storage.history.push(RelationshipLogEntry {
			user_id: "101".to_owned(),
			user_name: "Alice".to_owned(),
			change_type: "Удален из друзей".to_owned(),
			description: "Test removal".to_owned(),
		});

		let input = AppInvocation {
			invocation: Invocation {
				action: "show-history".to_owned(),
				storage: Some(serde_json::to_string(&storage).unwrap()),
				..Default::default()
			},
			..Default::default()
		};

		let output = handle(input);
		assert!(!output.panel.is_empty());

		// Test clear action
		let clear_input = AppInvocation {
			invocation: Invocation {
				action: "clear-history".to_owned(),
				storage: Some(serde_json::to_string(&storage).unwrap()),
				..Default::default()
			},
			..Default::default()
		};
		let clear_output = handle(clear_input);
		let cleared_storage: RelationshipStorage =
			serde_json::from_str(&clear_output.storage.expect("must save cleared storage")).unwrap();
		assert!(cleared_storage.history.is_empty());
	}
}
