use serein_extension_sdk::{export, AppInvocation, Element, Output};

pub fn handle(input: AppInvocation) -> Output {
	let action = input.invocation.action.as_str();
	match action {
		"show-permissions" => render_permissions(&input, None),
		"calc-permissions" => {
			let mask_str = input.invocation.value("bitmask").unwrap_or("0");
			render_permissions(&input, Some(mask_str))
		}
		_ => Output::default(),
	}
}

pub struct PermissionDef {
	pub bit: u64,
	pub name: &'static str,
	pub category: &'static str,
	pub description: &'static str,
}

pub const PERMISSIONS: &[PermissionDef] = &[
	// General
	PermissionDef {
		bit: 1 << 3,
		name: "Администратор (Administrator)",
		category: "🛡️ Общие серверные права",
		description: "Полный обход всех прав и ограничений каналов",
	},
	PermissionDef {
		bit: 1 << 5,
		name: "Управление сервером (Manage Guild)",
		category: "🛡️ Общие серверные права",
		description: "Изменение названия, региона и интеграций",
	},
	PermissionDef {
		bit: 1 << 28,
		name: "Управление ролями (Manage Roles)",
		category: "🛡️ Общие серверные права",
		description: "Создание и редактирование ролей ниже своей",
	},
	PermissionDef {
		bit: 1 << 4,
		name: "Управление каналами (Manage Channels)",
		category: "🛡️ Общие серверные права",
		description: "Создание, удаление и редактирование каналов",
	},
	PermissionDef {
		bit: 1 << 7,
		name: "Просмотр журнала аудита (View Audit Log)",
		category: "🛡️ Общие серверные права",
		description: "Просмотр действий модераторов и администраторов",
	},
	// Text
	PermissionDef {
		bit: 1 << 10,
		name: "Просмотр канала (View Channel)",
		category: "💬 Текстовые права канала",
		description: "Чтение сообщений и просмотр списка участников",
	},
	PermissionDef {
		bit: 1 << 11,
		name: "Отправка сообщений (Send Messages)",
		category: "💬 Текстовые права канала",
		description: "Отправка обычных текстовых сообщений",
	},
	PermissionDef {
		bit: 1 << 14,
		name: "Встраивание ссылок (Embed Links)",
		category: "💬 Текстовые права канала",
		description: "Рендеринг сниппетов и превью веб-страниц",
	},
	PermissionDef {
		bit: 1 << 15,
		name: "Прикрепление файлов (Attach Files)",
		category: "💬 Текстовые права канала",
		description: "Загрузка картинок, видео и документов",
	},
	PermissionDef {
		bit: 1 << 16,
		name: "Чтение истории сообщений (Read History)",
		category: "💬 Текстовые права канала",
		description: "Доступ к ранее отправленным сообщениям канала",
	},
	PermissionDef {
		bit: 1 << 17,
		name: "Упоминание @everyone (Mention Everyone)",
		category: "💬 Текстовые права канала",
		description: "Массовые пуш-уведомления участников",
	},
	PermissionDef {
		bit: 1 << 6,
		name: "Добавление реакций (Add Reactions)",
		category: "💬 Текстовые права канала",
		description: "Установка новых эмодзи-реакций на сообщения",
	},
	PermissionDef {
		bit: 1 << 13,
		name: "Управление сообщениями (Manage Messages)",
		category: "💬 Текстовые права канала",
		description: "Удаление и закрепление чужих сообщений",
	},
	// Voice
	PermissionDef {
		bit: 1 << 20,
		name: "Подключение к войсу (Connect)",
		category: "🎙️ Голосовые права",
		description: "Вход в голосовой канал и прослушивание",
	},
	PermissionDef {
		bit: 1 << 21,
		name: "Говорение (Speak)",
		category: "🎙️ Голосовые права",
		description: "Передача голоса через микрофон",
	},
	PermissionDef {
		bit: 1 << 9,
		name: "Стрим / Видео (Stream)",
		category: "🎙️ Голосовые права",
		description: "Трансляция экрана и включение веб-камеры",
	},
	PermissionDef {
		bit: 1 << 22,
		name: "Заглушение участников (Mute Members)",
		category: "🎙️ Голосовые права",
		description: "Серверный мут микрофона другим участникам",
	},
	PermissionDef {
		bit: 1 << 23,
		name: "Оглушение участников (Deafen Members)",
		category: "🎙️ Голосовые права",
		description: "Серверное отключение звука другим участникам",
	},
	PermissionDef {
		bit: 1 << 24,
		name: "Перемещение участников (Move Members)",
		category: "🎙️ Голосовые права",
		description: "Перевод участников между голосовыми каналами",
	},
];

fn parse_mask(s: &str) -> u64 {
	let s = s.trim();
	if let Some(hex) = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")) {
		u64::from_str_radix(hex, 16).unwrap_or(0)
	} else {
		s.parse::<u64>().unwrap_or(0)
	}
}

fn render_permissions(input: &AppInvocation, custom_mask: Option<&str>) -> Output {
	let mut panel = vec![
		Element::Heading {
			text: "Permissions Viewer".to_owned(),
		},
		Element::Text {
			text: "Инспектор и анализатор прав доступа Discord в активном канале.".to_owned(),
		},
		Element::Separator,
	];

	// Channel info from snapshot
	if let Some(app) = &input.app {
		if let Some(ch) = &app.channel_details {
			let send_badge = if ch.can_send { "[✔ Разрешено]" } else { "[✖ Запрещено]" };
			let history_badge = if ch.can_read_history { "[✔ Разрешено]" } else { "[✖ Запрещено]" };

			panel.push(Element::Heading {
				text: format!("Канал: #{} (ID: {})", ch.channel.name, ch.channel.id),
			});
			panel.push(Element::Row {
				children: vec![
					Element::Text {
						text: format!("Отправка сообщений: {send_badge}"),
					},
					Element::Text {
						text: format!("Чтение истории: {history_badge}"),
					},
				],
			});
			panel.push(Element::Separator);
		}

		if let Some(mem) = &app.member_details {
			if let Some(roles) = &mem.roles {
				panel.push(Element::Heading {
					text: format!("Доступные роли сервера ({})", roles.len()),
				});
				let role_names: Vec<String> = roles.iter().take(8).map(|r| r.name.clone()).collect();
				panel.push(Element::Text {
					text: format!("Роли: {}", role_names.join(", ")),
				});
				panel.push(Element::Separator);
			}
		}
	}

	// Bitmask calculator section
	let mask_val = custom_mask.map(parse_mask).unwrap_or(0);
	let current_input = custom_mask.unwrap_or("").to_owned();

	panel.push(Element::Heading {
		text: "Калькулятор битовой маски прав".to_owned(),
	});
	panel.push(Element::Text {
		text: "Введите десятичное число (например, 8 для Admin, 3072 для View+Send) или 0xHEX:"
			.to_owned(),
	});
	panel.push(Element::TextInput {
		id: "bitmask".to_owned(),
		label: "Маска прав (Permissions Bitmask)".to_owned(),
		value: current_input,
	});
	panel.push(Element::Button {
		id: "calc-permissions".to_owned(),
		label: "Рассчитать права".to_owned(),
	});

	if mask_val > 0 {
		panel.push(Element::Separator);
		panel.push(Element::Heading {
			text: format!("Анализ маски: {mask_val} (0x{mask_val:X})"),
		});

		let is_admin = (mask_val & (1 << 3)) != 0;
		if is_admin {
			panel.push(Element::Text {
				text: "★ АДМИНИСТРАТОР: Данная маска дает полный и безусловный доступ ко всем каналам!"
					.to_owned(),
			});
		}

		let mut current_cat = "";
		for perm in PERMISSIONS {
			if perm.category != current_cat {
				current_cat = perm.category;
				panel.push(Element::Heading {
					text: current_cat.to_owned(),
				});
			}

			let enabled = is_admin || (mask_val & perm.bit) != 0;
			let status = if enabled { "✔ ВКЛ" } else { "✖ выкл" };
			panel.push(Element::Text {
				text: format!("[{status}] {} — {}", perm.name, perm.description),
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
	use serein_extension_sdk::{
		AppSnapshot, ChannelDetailsSnapshot, ChannelSnapshot, Invocation,
	};

	#[test]
	fn test_default_permissions_view() {
		let input = AppInvocation {
			invocation: Invocation {
				action: "show-permissions".to_owned(),
				..Default::default()
			},
			app: Some(AppSnapshot {
				channel_details: Some(ChannelDetailsSnapshot {
					channel: ChannelSnapshot {
						id: "12345".to_owned(),
						guild_id: None,
						name: "general".to_owned(),
						kind: 0,
					},
					parent_id: None,
					position: 1,
					last_message_id: None,
					message_count: Some(100),
					recipients: Vec::new(),
					recipients_truncated: false,
					can_send: true,
					can_read_history: true,
				}),
				..Default::default()
			}),
			..Default::default()
		};

		let output = handle(input);
		assert!(!output.panel.is_empty());
		let has_channel_header = output.panel.iter().any(|e| match e {
			Element::Heading { text } => text.contains("general"),
			_ => false,
		});
		assert!(has_channel_header);
	}

	#[test]
	fn test_admin_bitmask_calculation() {
		let mut values = std::collections::BTreeMap::new();
		values.insert("bitmask".to_owned(), "8".to_owned());

		let input = AppInvocation {
			invocation: Invocation {
				action: "calc-permissions".to_owned(),
				values,
				..Default::default()
			},
			..Default::default()
		};

		let output = handle(input);
		let has_admin_notice = output.panel.iter().any(|e| match e {
			Element::Text { text } => text.contains("АДМИНИСТРАТОР"),
			_ => false,
		});
		assert!(has_admin_notice);
	}

	#[test]
	fn test_custom_hex_bitmask() {
		let mut values = std::collections::BTreeMap::new();
		values.insert("bitmask".to_owned(), "0xC00".to_owned()); // ViewChannel (1024) + SendMessages (2048) = 3072

		let input = AppInvocation {
			invocation: Invocation {
				action: "calc-permissions".to_owned(),
				values,
				..Default::default()
			},
			..Default::default()
		};

		let output = handle(input);
		let has_send_perm = output.panel.iter().any(|e| match e {
			Element::Text { text } => text.contains("[✔ ВКЛ] Отправка сообщений"),
			_ => false,
		});
		assert!(has_send_perm);
	}
}
