use serein_extension_sdk::{export, AppInvocation, Element, Output};

pub struct SettingCategory {
	pub id: &'static str,
	pub title: &'static str,
	pub description: &'static str,
	pub key_options: &'static [&'static str],
}

pub const SETTINGS_CATEGORIES: &[SettingCategory] = &[
	SettingCategory {
		id: "appearance",
		title: "Appearance & Themes",
		description: "Theme palette selection (Catppuccin, Tokyo Night, Nord), custom fonts, transparency.",
		key_options: &["Active Theme", "UI Scale", "Transparency Blur", "Font Family"],
	},
	SettingCategory {
		id: "notifications",
		title: "Notifications & Sounds",
		description: "Independent notification volume, sound toggles, idle auto-away timer, and unread badges.",
		key_options: &["Notification Volume", "Mute Inactive Sounds", "Idle Timeout (min)", "Desktop Popups"],
	},
	SettingCategory {
		id: "voice-audio",
		title: "Voice & Audio",
		description: "WASAPI low-latency audio, call timer, volume booster (up to 400%), secret ringtone, and voice status.",
		key_options: &["Volume Booster (400%)", "Call Timer Display", "Secret Ringtone (0.1%)", "Double Click Join"],
	},
	SettingCategory {
		id: "privacy-safety",
		title: "Privacy & Integrity",
		description: "Anonymise uploaded filenames, blur NSFW/spoilers, unmask phishing links, and no-blocked jumping.",
		key_options: &["Anonymise Filenames", "Blur NSFW Attachments", "Unmask Hidden Links", "Silent Typing"],
	},
	SettingCategory {
		id: "chat-composer",
		title: "Chat & Composer",
		description: "Full user tags in chatbox placeholder, character counter, quick templates, and timestamps generator.",
		key_options: &["Full User in Chatbox", "Character Counter", "Quick Replies", "Send Timestamps"],
	},
	SettingCategory {
		id: "extensions",
		title: "Extensions & Sandbox",
		description: "Wasm extension runtime diagnostics, 10M fuel budget, memory auditing, and catalog status.",
		key_options: &["Sandbox Fuel Limit (10M)", "Max I/O (256 KiB)", "Capability Ledger", "Catalog Sync"],
	},
];

pub fn filter_categories(query: &str) -> Vec<&'static SettingCategory> {
	let q = query.trim().to_lowercase();
	if q.is_empty() {
		return SETTINGS_CATEGORIES.iter().collect();
	}
	SETTINGS_CATEGORIES
		.iter()
		.filter(|cat| {
			cat.title.to_lowercase().contains(&q)
				|| cat.description.to_lowercase().contains(&q)
				|| cat.key_options.iter().any(|opt| opt.to_lowercase().contains(&q))
		})
		.collect()
}

pub fn generate_diagnostics_report(input: &AppInvocation) -> String {
	let mut report = String::from("⚙️ **VenSerein Client Configuration & Diagnostics Report**\n\n");

	if let Some(app) = &input.app {
		if let Some(profile) = &app.account_profile {
			report.push_str(&format!("• **User:** `{}` (ID: `{}`)\n", profile.user.name, profile.user.id));
			if let Some(own) = &profile.profile {
				if !own.bio.is_empty() {
					report.push_str(&format!("  *Bio:* {}\n", own.bio.trim()));
				}
				if !own.pronouns.is_empty() {
					report.push_str(&format!("  *Pronouns:* {}\n", own.pronouns.trim()));
				}
			}
		}

		if let Some(ctx) = &app.context {
			report.push_str(&format!("• **Gateway Status:** `{}`\n", if ctx.connected { "Connected" } else { "Offline" }));
			if let Some(channel) = &ctx.channel {
				report.push_str(&format!("• **Active Channel:** `{}` (ID: `{}`)\n", channel.name, channel.id));
				if let Some(guild_id) = &channel.guild_id {
					report.push_str(&format!("• **Active Guild ID:** `{guild_id}`\n"));
				}
			}
		}

		if let Some(settings) = &app.settings {
			report.push_str(&format!("• **UI Zoom:** `{}%` | **Sidebar:** `{}px` | **Animated GIFs:** `{}`\n", settings.zoom_percent, settings.sidebar_width, settings.animate_gifs));
		}

		if let Some(audio) = &app.audio_settings {
			report.push_str(&format!("• **Audio Input:** `{}%` | **Output:** `{}%` | **PTT:** `{}`\n", audio.input_percent, audio.output_percent, audio.push_to_talk));
		}

		if let Some(notif) = &app.notification_settings {
			report.push_str(&format!("• **Notifications Volume:** `{}%` | **Sounds:** `{}` | **Unread Badge:** `{}`\n", notif.volume, !notif.disable_sounds, notif.unread_badge));
		}
	} else {
		report.push_str("• *No app snapshot context provided by host.*\n");
	}

	report.push_str("\n**Active Architecture Modules:**\n");
	report.push_str("- Serein Wasm Extension Runtime (`wasm32-unknown-unknown`)\n");
	report.push_str("- Serein Rust Core Native Framework (`egui` + `wgpu`)\n");
	report.push_str("- Dual Delivery Pipeline (Wasm Sandboxing + High-Performance Core)\n");

	report
}

pub fn handle(input: AppInvocation) -> Output {
	let action = input.invocation.action.as_str();

	if action == "export-config" {
		let report = generate_diagnostics_report(&input);
		return Output {
			replacement: Some(report),
			panel: vec![
				Element::Heading {
					text: "Configuration Exported".into(),
				},
				Element::Text {
					text: "Client configuration report inserted into composer.".into(),
				},
			],
			..Default::default()
		};
	}

	let search_query = input
		.invocation
		.values
		.get("search-category")
		.map(|s| s.as_str())
		.unwrap_or_default();

	let categories = filter_categories(search_query);

	let mut panel = vec![
		Element::Heading {
			text: "Better Settings & Diagnostics".into(),
		},
		Element::Text {
			text: "Fast category lookup, client diagnostics, and configuration backup hub.".into(),
		},
		Element::TextInput {
			id: "search-category".into(),
			label: "Search settings, features or toggles:".into(),
			value: search_query.into(),
		},
		Element::Button {
			id: "export-config".into(),
			label: "Export Diagnostics Report to Composer".into(),
		},
		Element::Separator,
	];

	if categories.is_empty() {
		panel.push(Element::Text {
			text: format!("No settings found matching \"{search_query}\"."),
		});
	} else {
		for cat in categories {
			panel.push(Element::Heading {
				text: cat.title.into(),
			});
			panel.push(Element::Text {
				text: cat.description.into(),
			});
			let opts = cat.key_options.join(" • ");
			panel.push(Element::Text {
				text: format!("*Options:* {opts}"),
			});
			panel.push(Element::Separator);
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
	use serein_extension_sdk::{
		AccountProfileSnapshot, AppContextSnapshot, AppSnapshot, AudioSettingsSnapshot,
		ChannelSnapshot, Invocation, LocalSettingsSnapshot, NotificationSettingsSnapshot,
		OwnProfileSnapshot, UserSnapshot,
	};

	#[test]
	fn filters_settings_categories() {
		let all = filter_categories("");
		assert_eq!(all.len(), SETTINGS_CATEGORIES.len());

		let appearance = filter_categories("appearance");
		assert_eq!(appearance.len(), 1);
		assert_eq!(appearance[0].id, "appearance");

		let booster = filter_categories("booster");
		assert_eq!(booster.len(), 1);
		assert_eq!(booster[0].id, "voice-audio");

		let missing = filter_categories("nonexistent-key-1234");
		assert!(missing.is_empty());
	}

	#[test]
	fn generates_diagnostics_report() {
		let app = AppSnapshot {
			account_profile: Some(AccountProfileSnapshot {
				user: UserSnapshot {
					id: "42".into(),
					name: "SereinUser".into(),
				},
				avatar: None,
				profile: Some(OwnProfileSnapshot {
					display_name: Some("Serein User".into()),
					bio: "Coding in Rust".into(),
					pronouns: "they/them".into(),
				}),
			}),
			context: Some(AppContextSnapshot {
				connected: true,
				user: None,
				channel: Some(ChannelSnapshot {
					id: "101".into(),
					name: "general".into(),
					guild_id: Some("202".into()),
					kind: 0,
				}),
			}),
			settings: Some(LocalSettingsSnapshot {
				zoom_percent: 100,
				sidebar_width: 240,
				show_members: true,
				animate_gifs: true,
				hide_media_links: false,
				smooth_scrolling: Some(true),
				scroll_speed_percent: Some(100),
			}),
			audio_settings: Some(AudioSettingsSnapshot {
				input_percent: 100,
				output_percent: 85,
				push_to_talk: false,
				input_profile: "standard".into(),
				suppression: "krisp".into(),
				suppression_level: 1,
				echo_cancellation: true,
				automatic_gain: true,
				sensitivity_db: None,
			}),
			notification_settings: Some(NotificationSettingsSnapshot {
				new_message: true,
				current_channel: false,
				incoming_ring: true,
				outgoing_ring: true,
				disable_sounds: false,
				unread_badge: true,
				mute: true,
				unmute: true,
				deafen: true,
				undeafen: true,
				camera_on: true,
				screen_share_on: true,
				user_join: true,
				user_leave: true,
				volume: 70,
			}),
			..Default::default()
		};

		let input = AppInvocation {
			invocation: Invocation {
				action: "export-config".into(),
				..Default::default()
			},
			app: Some(app),
			..Default::default()
		};

		let report = generate_diagnostics_report(&input);
		assert!(report.contains("SereinUser"));
		assert!(report.contains("general"));
		assert!(report.contains("202"));
		assert!(report.contains("85%"));
		assert!(report.contains("70%"));

		let output = handle(input);
		assert!(output.replacement.is_some());
		assert!(output.replacement.unwrap().contains("SereinUser"));
	}

	#[test]
	fn handles_settings_hub_panel() {
		let mut values = std::collections::BTreeMap::new();
		values.insert("search-category".into(), "privacy".into());

		let input = AppInvocation {
			invocation: Invocation {
				action: "settings-hub".into(),
				values,
				..Default::default()
			},
			app: None,
			..Default::default()
		};

		let output = handle(input);
		assert!(!output.panel.is_empty());
		let has_privacy = output.panel.iter().any(|elem| match elem {
			Element::Heading { text } => text.contains("Privacy"),
			_ => false,
		});
		assert!(has_privacy);
	}
}
