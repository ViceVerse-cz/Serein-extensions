use serein_extension_sdk::{
	export, AppInvocation, Element, Output, RelationshipKind, RelationshipSnapshot,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SortDirection {
	Ascending,
	Descending,
}

pub fn filter_and_sort_requests(
	items: &[RelationshipSnapshot],
	incoming: bool,
	direction: SortDirection,
) -> Vec<RelationshipSnapshot> {
	let target_kind = if incoming {
		RelationshipKind::IncomingRequest
	} else {
		RelationshipKind::OutgoingRequest
	};

	let mut filtered: Vec<RelationshipSnapshot> = items
		.iter()
		.filter(|r| r.kind == target_kind)
		.cloned()
		.collect();

	match direction {
		SortDirection::Ascending => {
			filtered.sort_by(|a, b| a.user.name.to_lowercase().cmp(&b.user.name.to_lowercase()));
		}
		SortDirection::Descending => {
			filtered.sort_by(|a, b| b.user.name.to_lowercase().cmp(&a.user.name.to_lowercase()));
		}
	}

	filtered
}

pub fn handle(input: AppInvocation) -> Output {
	let action = input.invocation.action.as_str();

	let direction = if action == "sort-za" {
		SortDirection::Descending
	} else {
		SortDirection::Ascending
	};

	let relationships = input
		.app
		.as_ref()
		.and_then(|app| app.relationships.as_ref());

	let Some(rel) = relationships else {
		return Output {
			panel: vec![
				Element::Heading {
					text: "Friend Requests".into(),
				},
				Element::Text {
					text: "No relationship data available or permission not granted.".into(),
				},
			],
			..Default::default()
		};
	};

	let incoming = filter_and_sort_requests(&rel.items, true, direction);
	let outgoing = filter_and_sort_requests(&rel.items, false, direction);

	let dir_label = match direction {
		SortDirection::Ascending => "Sorted A \u{2192} Z",
		SortDirection::Descending => "Sorted Z \u{2192} A",
	};

	let mut panel = vec![
		Element::Heading {
			text: "Sort Friend Requests".into(),
		},
		Element::Text {
			text: format!("Current Order: {dir_label}"),
		},
		Element::Row {
			children: vec![
				Element::Button {
					id: "sort-az".into(),
					label: "Sort A \u{2192} Z".into(),
				},
				Element::Button {
					id: "sort-za".into(),
					label: "Sort Z \u{2192} A".into(),
				},
			],
		},
		Element::Separator,
		Element::Heading {
			text: format!("Incoming Requests ({})", incoming.len()),
		},
	];

	if incoming.is_empty() {
		panel.push(Element::Text {
			text: "No incoming friend requests.".into(),
		});
	} else {
		for item in incoming.iter().take(20) {
			panel.push(Element::Text {
				text: format!("\u{2022} @{} (ID: {})", item.user.name, item.user.id),
			});
		}
	}

	panel.push(Element::Separator);
	panel.push(Element::Heading {
		text: format!("Outgoing Requests ({})", outgoing.len()),
	});

	if outgoing.is_empty() {
		panel.push(Element::Text {
			text: "No outgoing friend requests.".into(),
		});
	} else {
		for item in outgoing.iter().take(20) {
			panel.push(Element::Text {
				text: format!("\u{2022} @{} (ID: {})", item.user.name, item.user.id),
			});
		}
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
	use serein_extension_sdk::{AppSnapshot, Invocation, RelationshipsSnapshot, UserSnapshot};

	fn sample_relationships() -> Vec<RelationshipSnapshot> {
		vec![
			RelationshipSnapshot {
				user: UserSnapshot {
					id: "101".into(),
					name: "Zack".into(),
				},
				kind: RelationshipKind::IncomingRequest,
			},
			RelationshipSnapshot {
				user: UserSnapshot {
					id: "102".into(),
					name: "Alice".into(),
				},
				kind: RelationshipKind::IncomingRequest,
			},
			RelationshipSnapshot {
				user: UserSnapshot {
					id: "103".into(),
					name: "Bob".into(),
				},
				kind: RelationshipKind::OutgoingRequest,
			},
			RelationshipSnapshot {
				user: UserSnapshot {
					id: "104".into(),
					name: "Charlie".into(),
				},
				kind: RelationshipKind::Friend,
			},
		]
	}

	#[test]
	fn filters_and_sorts_ascending() {
		let items = sample_relationships();
		let incoming = filter_and_sort_requests(&items, true, SortDirection::Ascending);
		assert_eq!(incoming.len(), 2);
		assert_eq!(incoming[0].user.name, "Alice");
		assert_eq!(incoming[1].user.name, "Zack");
	}

	#[test]
	fn filters_and_sorts_descending() {
		let items = sample_relationships();
		let incoming = filter_and_sort_requests(&items, true, SortDirection::Descending);
		assert_eq!(incoming.len(), 2);
		assert_eq!(incoming[0].user.name, "Zack");
		assert_eq!(incoming[1].user.name, "Alice");
	}

	#[test]
	fn handles_app_invocation_panel() {
		let app = AppSnapshot {
			relationships: Some(RelationshipsSnapshot {
				items: sample_relationships(),
				truncated: false,
				friends_known: true,
				requests_known: true,
				restricted_known: true,
			}),
			..Default::default()
		};

		let input = AppInvocation {
			invocation: Invocation {
				action: "sort-az".into(),
				..Default::default()
			},
			app: Some(app),
			..Default::default()
		};

		let output = handle(input);
		assert!(!output.panel.is_empty());
	}
}
