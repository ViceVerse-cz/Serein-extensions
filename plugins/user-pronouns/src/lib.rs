use serein_extension_sdk::{export, AppInvocation, Element, Output};

pub fn format_pronouns_badge(raw: &str) -> String {
	let trimmed = raw.trim();
	if trimmed.is_empty() {
		return String::new();
	}
	if trimmed.starts_with('[') && trimmed.ends_with(']') {
		trimmed.to_string()
	} else if trimmed.starts_with('(') && trimmed.ends_with(')') {
		format!("[{}]", &trimmed[1..trimmed.len() - 1].trim())
	} else {
		format!("[{trimmed}]")
	}
}

pub fn handle(input: AppInvocation) -> Output {
	let action = input.invocation.action.as_str();

	let mut my_pronouns = String::new();
	if let Some(app) = &input.app {
		if let Some(account) = &app.account_profile {
			if let Some(profile) = &account.profile {
				my_pronouns = profile.pronouns.trim().to_string();
			}
		}
	}

	let my_badge = if !my_pronouns.is_empty() {
		format_pronouns_badge(&my_pronouns)
	} else {
		"[they/them]".to_string()
	};

	if action == "insert-pronouns" {
		return Output {
			replacement: Some(my_badge.clone()),
			panel: vec![
				Element::Heading {
					text: "Pronouns Added".into(),
				},
				Element::Text {
					text: format!("Inserted {my_badge} into message composer."),
				},
			],
			..Default::default()
		};
	}

	let mut panel = vec![
		Element::Heading {
			text: "User Pronouns".into(),
		},
		Element::Text {
			text: if !my_pronouns.is_empty() {
				format!("Your active pronouns: {my_badge}")
			} else {
				"No pronouns set in your account profile.".into()
			},
		},
		Element::Button {
			id: "insert-pronouns".into(),
			label: format!("Insert {my_badge} into Composer"),
		},
		Element::Separator,
		Element::Heading {
			text: "Server Member Pronouns".into(),
		},
	];

	let mut member_count = 0;
	if let Some(app) = &input.app {
		if let Some(details) = &app.member_details {
			for member in &details.items {
				if let Some(profile) = &member.profile {
					if !profile.pronouns.trim().is_empty() {
						let badge = format_pronouns_badge(&profile.pronouns);
						let name = member.nick.as_deref().unwrap_or(&member.display_name);
						panel.push(Element::Text {
							text: format!("• @{name} {badge}"),
						});
						member_count += 1;
					}
				}
			}
		}
	}

	if member_count == 0 {
		panel.push(Element::Text {
			text: "No member pronouns discovered in current channel.".into(),
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
		AccountProfileSnapshot, AppSnapshot, Invocation, MemberDetailSnapshot,
		MemberDetailsSnapshot, MemberProfileSnapshot, OwnProfileSnapshot, UserSnapshot,
	};

	#[test]
	fn formats_pronoun_badges_cleanly() {
		assert_eq!(format_pronouns_badge(""), "");
		assert_eq!(format_pronouns_badge("she/her"), "[she/her]");
		assert_eq!(format_pronouns_badge("(he/him)"), "[he/him]");
		assert_eq!(format_pronouns_badge("[they/them]"), "[they/them]");
	}

	#[test]
	fn inserts_own_pronouns_into_composer() {
		let app = AppSnapshot {
			account_profile: Some(AccountProfileSnapshot {
				user: UserSnapshot {
					id: "123".into(),
					name: "Luna".into(),
				},
				avatar: None,
				profile: Some(OwnProfileSnapshot {
					display_name: Some("Luna".into()),
					bio: "Rust developer".into(),
					pronouns: "she/her".into(),
				}),
			}),
			..Default::default()
		};

		let input = AppInvocation {
			invocation: Invocation {
				action: "insert-pronouns".into(),
				..Default::default()
			},
			app: Some(app),
			..Default::default()
		};

		let output = handle(input);
		assert_eq!(output.replacement.as_deref(), Some("[she/her]"));
	}

	#[test]
	fn lists_member_pronouns_in_panel() {
		let app = AppSnapshot {
			member_details: Some(MemberDetailsSnapshot {
				channel_id: "1001".into(),
				guild_id: "2001".into(),
				items: vec![MemberDetailSnapshot {
					user: UserSnapshot {
						id: "301".into(),
						name: "Alex".into(),
					},
					nick: Some("Alex Mercer".into()),
					display_name: "Alex".into(),
					role_ids: vec![],
					roles_truncated: false,
					profile: Some(MemberProfileSnapshot {
						nick: None,
						avatar: None,
						bio: "".into(),
						pronouns: "they/them".into(),
						joined_at: None,
					}),
				}],
				truncated: false,
				roles: None,
				roles_truncated: false,
			}),
			..Default::default()
		};

		let input = AppInvocation {
			invocation: Invocation {
				action: "view-pronouns".into(),
				..Default::default()
			},
			app: Some(app),
			..Default::default()
		};

		let output = handle(input);
		assert!(output.panel.iter().any(|e| match e {
			Element::Text { text } => text.contains("they/them"),
			_ => false,
		}));
	}
}
