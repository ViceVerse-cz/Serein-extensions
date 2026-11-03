use serein_extension_sdk::{export, AppInvocation, Element, Output, RelationshipKind};

pub fn handle(input: AppInvocation) -> Output {
	let mut panel = vec![
		Element::Heading {
			text: "Implicit & Extended Relationships".into(),
		},
		Element::Separator,
	];

	let Some(app) = &input.app else {
		panel.push(Element::Text {
			text: "App state unavailable.".into(),
		});
		return Output {
			panel,
			..Default::default()
		};
	};

	let relationships = match &app.relationships {
		Some(r) => &r.items,
		None => {
			panel.push(Element::Text {
				text: "No relationship data available in this context.".into(),
			});
			return Output {
				panel,
				..Default::default()
			};
		}
	};

	let mut friends = Vec::new();
	let mut incoming = Vec::new();
	let mut outgoing = Vec::new();
	let mut blocked = Vec::new();
	let mut ignored = Vec::new();

	for rel in relationships {
		match rel.kind {
			RelationshipKind::Friend => friends.push(&rel.user),
			RelationshipKind::IncomingRequest => incoming.push(&rel.user),
			RelationshipKind::OutgoingRequest => outgoing.push(&rel.user),
			RelationshipKind::Blocked => blocked.push(&rel.user),
			RelationshipKind::Ignored => ignored.push(&rel.user),
		}
	}

	let summary = format!(
		"Audit: {} Friends | {} Incoming | {} Outgoing | {} Blocked | {} Ignored",
		friends.len(),
		incoming.len(),
		outgoing.len(),
		blocked.len(),
		ignored.len(),
	);
	panel.push(Element::Text { text: summary });

	if !incoming.is_empty() {
		panel.push(Element::Heading {
			text: format!("Pending Incoming Requests ({})", incoming.len()),
		});
		for user in incoming.iter().take(10) {
			panel.push(Element::Text {
				text: format!("- {} (ID: {})", user.name, user.id),
			});
		}
	}

	if !outgoing.is_empty() {
		panel.push(Element::Heading {
			text: format!("Pending Outgoing Requests ({})", outgoing.len()),
		});
		for user in outgoing.iter().take(10) {
			panel.push(Element::Text {
				text: format!("- {} (ID: {})", user.name, user.id),
			});
		}
	}

	if !ignored.is_empty() {
		panel.push(Element::Heading {
			text: format!("Ignored Relationships ({})", ignored.len()),
		});
		for user in ignored.iter().take(10) {
			panel.push(Element::Text {
				text: format!("- {} (ID: {})", user.name, user.id),
			});
		}
	}

	if !blocked.is_empty() {
		panel.push(Element::Heading {
			text: format!("Blocked Users ({})", blocked.len()),
		});
		for user in blocked.iter().take(5) {
			panel.push(Element::Text {
				text: format!("- {} (ID: {})", user.name, user.id),
			});
		}
	}

	if incoming.is_empty() && outgoing.is_empty() && ignored.is_empty() {
		panel.push(Element::Text {
			text: "No pending or implicit relationship requests found. All connections are normal."
				.into(),
		});
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
	use serein_extension_sdk::{
		AppSnapshot, Invocation, RelationshipSnapshot, RelationshipsSnapshot, UserSnapshot,
	};

	#[test]
	fn audits_all_relationship_types() {
		let app = AppSnapshot {
			relationships: Some(RelationshipsSnapshot {
				items: vec![
					RelationshipSnapshot {
						user: UserSnapshot {
							id: "101".into(),
							name: "Alice".into(),
						},
						kind: RelationshipKind::Friend,
					},
					RelationshipSnapshot {
						user: UserSnapshot {
							id: "102".into(),
							name: "Bob".into(),
						},
						kind: RelationshipKind::IncomingRequest,
					},
					RelationshipSnapshot {
						user: UserSnapshot {
							id: "103".into(),
							name: "Charlie".into(),
						},
						kind: RelationshipKind::OutgoingRequest,
					},
				],
				truncated: false,
				friends_known: true,
				requests_known: true,
				restricted_known: true,
			}),
			..Default::default()
		};

		let input = AppInvocation {
			invocation: Invocation::default(),
			app: Some(app),
			..Default::default()
		};

		let output = handle(input);
		assert!(output.panel.len() >= 4);

		let texts: Vec<String> = output
			.panel
			.iter()
			.filter_map(|e| match e {
				Element::Text { text } => Some(text.clone()),
				Element::Heading { text } => Some(text.clone()),
				_ => None,
			})
			.collect();

		assert!(texts.iter().any(|t| t.contains("1 Incoming")));
		assert!(texts.iter().any(|t| t.contains("Bob (ID: 102)")));
		assert!(texts.iter().any(|t| t.contains("Charlie (ID: 103)")));
	}
}
