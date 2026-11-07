use serein_extension_sdk::{export, Element, Invocation, Output};

pub fn handle(input: Invocation) -> Output {
	let action = input.action.as_str();
	match action {
		"preview-draft" => render_preview(&input),
		"apply-format" => normalize_and_apply(&input),
		_ => Output::default(),
	}
}

fn render_preview(input: &Invocation) -> Output {
	let draft = input.composer.as_deref().unwrap_or("").trim();
	if draft.is_empty() {
		return Output {
			panel: vec![
				Element::Heading {
					text: "Предпросмотр сообщения".to_owned(),
				},
				Element::Text {
					text: "Драфт пуст. Введите текст сообщения в поле ввода, чтобы увидеть его предпросмотр."
						.to_owned(),
				},
			],
			..Default::default()
		};
	}

	let char_count = draft.chars().count();
	let word_count = draft.split_whitespace().count();
	let line_count = draft.lines().count();

	let has_codeblocks = draft.contains("```");
	let has_spoilers = draft.contains("||");
	let has_quotes = draft.lines().any(|l| l.trim_start().starts_with('>'));
	let has_links = draft.contains("http://") || draft.contains("https://");

	let mut tags = Vec::new();
	if has_codeblocks {
		tags.push("Код (Codeblock)");
	}
	if has_spoilers {
		tags.push("Спойлеры (Spoilers)");
	}
	if has_quotes {
		tags.push("Цитаты (Quotes)");
	}
	if has_links {
		tags.push("Ссылки (URLs)");
	}
	let tags_str = if tags.is_empty() {
		"Стандартный текст".to_owned()
	} else {
		tags.join(", ")
	};

	let limit_pct = (char_count * 100) / 2000;
	let limit_str = format!("{char_count} / 2000 симв. ({limit_pct}%)");

	let mut panel = vec![
		Element::Heading {
			text: "Предпросмотр сообщения".to_owned(),
		},
		Element::Row {
			children: vec![
				Element::Text {
					text: format!("Лимит: {limit_str}"),
				},
				Element::Text {
					text: format!("Слов: {word_count} • Строк: {line_count}"),
				},
			],
		},
		Element::Text {
			text: format!("Форматирование: {tags_str}"),
		},
		Element::Separator,
		Element::Heading {
			text: "Рендеринг содержимого:".to_owned(),
		},
	];

	// Parse lines and render preview
	let mut in_codeblock = false;
	let mut codeblock_buf = Vec::new();

	for line in draft.lines() {
		if line.trim_start().starts_with("```") {
			if in_codeblock {
				// Close codeblock
				panel.push(Element::Text {
					text: format!("┌─ [Code Block]\n│ {}\n└─────────────", codeblock_buf.join("\n│ ")),
				});
				codeblock_buf.clear();
				in_codeblock = false;
			} else {
				in_codeblock = true;
			}
			continue;
		}

		if in_codeblock {
			codeblock_buf.push(line);
			continue;
		}

		if line.trim_start().starts_with('#') {
			panel.push(Element::Heading {
				text: line.trim_start_matches('#').trim().to_owned(),
			});
		} else if line.trim_start().starts_with('>') {
			panel.push(Element::Text {
				text: format!("▍ {}", line.trim_start_matches('>').trim()),
			});
		} else if !line.trim().is_empty() {
			panel.push(Element::Text {
				text: line.to_owned(),
			});
		}
	}

	if in_codeblock && !codeblock_buf.is_empty() {
		panel.push(Element::Text {
			text: format!("┌─ [Code Block]\n│ {}\n└─────────────", codeblock_buf.join("\n│ ")),
		});
	}

	panel.push(Element::Separator);
	panel.push(Element::Button {
		id: "apply-format".to_owned(),
		label: "Нормализовать форматирование".to_owned(),
	});

	Output {
		panel,
		..Default::default()
	}
}

fn normalize_and_apply(input: &Invocation) -> Output {
	let draft = input.composer.as_deref().unwrap_or("");
	let mut normalized_lines = Vec::new();
	let mut prev_empty = false;

	for line in draft.lines() {
		let trimmed = line.trim_end();
		if trimmed.is_empty() {
			if !prev_empty {
				normalized_lines.push("");
				prev_empty = true;
			}
		} else {
			normalized_lines.push(trimmed);
			prev_empty = false;
		}
	}

	let result = normalized_lines.join("\n").trim().to_owned();
	Output {
		replacement: Some(result),
		panel: vec![
			Element::Heading {
				text: "Форматирование нормализовано".to_owned(),
			},
			Element::Text {
				text: "Лишние пробелы в концах строк и повторяющиеся пустые строки удалены.".to_owned(),
			},
		],
		..Default::default()
	}
}

export!(handle);

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_empty_draft() {
		let input = Invocation {
			action: "preview-draft".to_owned(),
			composer: Some("".to_owned()),
			..Default::default()
		};
		let output = handle(input);
		assert_eq!(output.panel.len(), 2);
	}

	#[test]
	fn test_markdown_preview() {
		let draft = "# Heading\n> Quote line\n```rust\nfn main() {}\n```\nPlain text with https://example.com";
		let input = Invocation {
			action: "preview-draft".to_owned(),
			composer: Some(draft.to_owned()),
			..Default::default()
		};
		let output = handle(input);
		assert!(output.panel.len() >= 6);
	}

	#[test]
	fn test_normalization() {
		let raw = "line 1   \n\n\n\nline 2   \n";
		let input = Invocation {
			action: "apply-format".to_owned(),
			composer: Some(raw.to_owned()),
			..Default::default()
		};
		let output = handle(input);
		assert_eq!(output.replacement.as_deref(), Some("line 1\n\nline 2"));
	}
}
