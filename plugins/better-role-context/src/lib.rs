use serein_extension_sdk::{export, AppInvocation, Element, MemberRoleSnapshot, Output};

pub fn format_role_hex(color: u32) -> String {
	if color == 0 {
		"Default".to_string()
	} else {
		format!("#{:06X}", color)
	}
}

pub fn sort_roles_by_hierarchy(roles: &[MemberRoleSnapshot]) -> Vec<MemberRoleSnapshot> {
	let mut sorted = roles.to_vec();
	sorted.sort_by(|a, b| b.position.cmp(&a.position));
	sorted
}

pub fn handle(input: AppInvocation) -> Output {
	let action = input.invocation.action.as_str();

	let roles = input
		.app
		.as_ref()
		.and_then(|app| app.member_details.as_ref())
		.and_then(|details| details.roles.as_ref());

	let Some(roles_list) = roles else {
		return Output {
			panel: vec![
				Element::Heading {
					text: "Server Roles".into(),
				},
				Element::Text {
					text: "No role catalog available in active channel context.".into(),
				},
			],
			..Default::default()
		};
	};

	let sorted = sort_roles_by_hierarchy(roles_list);

	let formatted_markdown = sorted
		.iter()
		.map(|r| format!("• **{}** (ID: `{}`, Color: `{}`)", r.name, r.id, format_role_hex(r.color)))
		.collect::<Vec<_>>()
		.join("\n");

	if action == "copy-roles" {
		return Output {
			replacement: Some(format!("📋 **Server Roles Hierarchy:**\n{formatted_markdown}")),
			panel: vec![
				Element::Heading {
					text: "Roles List Copied".into(),
				},
				Element::Text {
					text: format!("Added {} server roles to composer.", sorted.len()),
				},
			],
			..Default::default()
		};
	}

	let mut panel = vec![
		Element::Heading {
			text: format!("Server Roles ({})", sorted.len()),
		},
		Element::Text {
			text: "Roles sorted by server hierarchy (highest to lowest):".into(),
		},
		Element::Button {
			id: "copy-roles".into(),
			label: "Copy Roles List into Composer".into(),
		},
		Element::Separator,
	];

	for role in sorted.iter().take(25) {
		panel.push(Element::Text {
			text: format!("• **{}** — Color: `{}` (Pos: {})", role.name, format_role_hex(role.color), role.position),
		});
	}

	if sorted.len() > 25 {
		panel.push(Element::Text {
			text: format!("…and {} more roles.", sorted.len() - 25),
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
	use serein_extension_sdk::{AppSnapshot, Invocation, MemberDetailsSnapshot};

	fn mock_roles() -> Vec<MemberRoleSnapshot> {
		vec![
			MemberRoleSnapshot {
				id: "10".into(),
				name: "Member".into(),
				color: 0,
				position: 1,
			},
			MemberRoleSnapshot {
				id: "20".into(),
				name: "Admin".into(),
				color: 0xFF5733,
				position: 10,
			},
			MemberRoleSnapshot {
				id: "30".into(),
				name: "Moderator".into(),
				color: 0x33FF57,
				position: 5,
			},
		]
	}

	#[test]
	fn formats_hex_color() {
		assert_eq!(format_role_hex(0), "Default");
		assert_eq!(format_role_hex(0xFF5733), "#FF5733");
		assert_eq!(format_role_hex(0x00FF00), "#00FF00");
	}

	#[test]
	fn sorts_roles_descending_by_position() {
		let roles = mock_roles();
		let sorted = sort_roles_by_hierarchy(&roles);
		assert_eq!(sorted[0].name, "Admin");
		assert_eq!(sorted[1].name, "Moderator");
		assert_eq!(sorted[2].name, "Member");
	}

	#[test]
	fn copies_roles_to_composer() {
		let app = AppSnapshot {
			member_details: Some(MemberDetailsSnapshot {
				channel_id: "1".into(),
				guild_id: "2".into(),
				items: vec![],
				truncated: false,
				roles: Some(mock_roles()),
				roles_truncated: false,
			}),
			..Default::default()
		};

		let input = AppInvocation {
			invocation: Invocation {
				action: "copy-roles".into(),
				..Default::default()
			},
			app: Some(app),
			..Default::default()
		};

		let output = handle(input);
		assert!(output.replacement.is_some());
		let text = output.replacement.unwrap();
		assert!(text.contains("Admin"));
		assert!(text.contains("#FF5733"));
	}
}
