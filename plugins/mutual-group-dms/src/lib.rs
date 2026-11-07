use serein_extension_sdk::{export, AppInvocation, Element, Output};

pub fn handle(input: AppInvocation) -> Output {
	let action = input.invocation.action.as_str();
	match action {
		"show-mutual-groups" | "refresh-groups" => {
			let filter_query = input.invocation.value("group-filter").unwrap_or("").trim();
			render_groups_panel(&input, filter_query)
		}
		_ => Output::default(),
	}
}

fn render_groups_panel(input: &AppInvocation, filter: &str) -> Output {
	let mut panel = vec![
		Element::Heading {
			text: "Mutual Group DMs".to_owned(),
		},
		Element::Text {
			text: "Инспектор доступных групповых чатов (Group DMs) и совместных бесед.".to_owned(),
		},
		Element::Separator,
	];

	let mut groups = Vec::new();
	if let Some(app) = &input.app {
		if let Some(dir) = &app.channels {
			for ch in &dir.items {
				// kind 3 is Discord Group DM (GROUP_DM)
				if ch.kind == 3 {
					if filter.is_empty()
						|| ch.name.to_lowercase().contains(&filter.to_lowercase())
						|| ch.id.contains(filter)
					{
						groups.push(ch);
					}
				}
			}
		}
	}

	if groups.is_empty() {
		panel.push(Element::Text {
			text: if filter.is_empty() {
				"Групповые чаты (Group DMs) не найдены в текущем списке каналов клиента.".to_owned()
			} else {
				format!("По запросу \"{filter}\" групповых чатов не найдено.")
			},
		});
	} else {
		panel.push(Element::Heading {
			text: format!("Найдено групповых чатов: {}", groups.len()),
		});
		for g in &groups {
			panel.push(Element::Text {
				text: format!("👥 {} (ID: {})", g.name, g.id),
			});
		}
	}

	panel.push(Element::Separator);
	panel.push(Element::TextInput {
		id: "group-filter".to_owned(),
		label: "Фильтр по названию или ID группы:".to_owned(),
		value: filter.to_owned(),
	});
	panel.push(Element::Button {
		id: "refresh-groups".to_owned(),
		label: "Обновить список групп".to_owned(),
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
		AppSnapshot, ChannelDirectorySnapshot, ChannelSnapshot, Invocation,
	};

	#[test]
	fn test_group_dm_filtering() {
		let dir = ChannelDirectorySnapshot {
			items: vec![
				ChannelSnapshot {
					id: "101".to_owned(),
					guild_id: None,
					name: "Gaming Squad".to_owned(),
					kind: 3, // Group DM
				},
				ChannelSnapshot {
					id: "102".to_owned(),
					guild_id: Some("server1".to_owned()),
					name: "general".to_owned(),
					kind: 0, // Guild Text
				},
				ChannelSnapshot {
					id: "103".to_owned(),
					guild_id: None,
					name: "Study Group".to_owned(),
					kind: 3, // Group DM
				},
			],
			truncated: false,
		};

		let input = AppInvocation {
			invocation: Invocation {
				action: "show-mutual-groups".to_owned(),
				..Default::default()
			},
			app: Some(AppSnapshot {
				channels: Some(dir),
				..Default::default()
			}),
			..Default::default()
		};

		let output = handle(input);
		assert!(!output.panel.is_empty());

		// Verify 2 Group DMs are listed and guild text channel is excluded
		let has_gaming = output.panel.iter().any(|e| match e {
			Element::Text { text } => text.contains("Gaming Squad"),
			_ => false,
		});
		let has_study = output.panel.iter().any(|e| match e {
			Element::Text { text } => text.contains("Study Group"),
			_ => false,
		});
		let has_general = output.panel.iter().any(|e| match e {
			Element::Text { text } => text.contains("general"),
			_ => false,
		});

		assert!(has_gaming);
		assert!(has_study);
		assert!(!has_general, "Guild channels must be excluded from group DMs");
	}
}
