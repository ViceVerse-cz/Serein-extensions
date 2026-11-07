use serein_extension_sdk::{export, AppInvocation, Element, Output};

pub fn handle(input: AppInvocation) -> Output {
	let action = input.invocation.action.as_str();

	let (user_id, username) = if let Some(app) = &input.app {
		if let Some(profile) = &app.account_profile {
			(profile.user.id.clone(), profile.user.name.clone())
		} else {
			("0".into(), "user".into())
		}
	} else {
		("0".into(), "user".into())
	};

	let user_url = format!("https://discord.com/users/{user_id}");
	let short_invite = format!("https://discord.gg/users/{user_id}");
	let markdown_invite = format!("👋 **Add me on Discord:** [@{username}]({user_url})");

	if action == "create-invite" {
		return Output {
			replacement: Some(markdown_invite),
			panel: vec![
				Element::Heading {
					text: "Friend Invite Link Created".into(),
				},
				Element::Text {
					text: format!("Added invite for @{username} to your composer."),
				},
			],
			..Default::default()
		};
	}

	let panel = vec![
		Element::Heading {
			text: "Discord Friend Invites".into(),
		},
		Element::Text {
			text: format!("Generate and share direct friend links for @{username}:"),
		},
		Element::Separator,
		Element::Text {
			text: format!("• Profile URL: {user_url}"),
		},
		Element::Text {
			text: format!("• Short Invite: {short_invite}"),
		},
		Element::Text {
			text: format!("• Markdown: {markdown_invite}"),
		},
		Element::Button {
			id: "create-invite".into(),
			label: "Insert Invite Link into Composer".into(),
		},
	];

	Output {
		panel,
		..Default::default()
	}
}

export!(handle);

#[cfg(test)]
mod tests {
	use super::*;
	use serein_extension_sdk::{AccountProfileSnapshot, AppSnapshot, Invocation, UserSnapshot};

	#[test]
	fn generates_invite_links_from_profile() {
		let app = AppSnapshot {
			account_profile: Some(AccountProfileSnapshot {
				user: UserSnapshot {
					id: "123456789".into(),
					name: "Luna".into(),
				},
				avatar: None,
				profile: None,
			}),
			..Default::default()
		};

		let input = AppInvocation {
			invocation: Invocation {
				action: "create-invite".into(),
				..Default::default()
			},
			app: Some(app),
			..Default::default()
		};

		let output = handle(input);
		assert!(output.replacement.is_some());
		let text = output.replacement.unwrap();
		assert!(text.contains("123456789"));
		assert!(text.contains("@Luna"));
	}
}
