use serein_extension_sdk::{export, Element, Invocation, Output};

pub fn handle(input: Invocation) -> Output {
	let action = input.action.as_str();
	match action {
		"search-image" => {
			let message = input.selected_message.as_deref().unwrap_or("");
			let url = extract_image_url(message);
			render_search_panel(url.as_deref())
		}
		"calc-engines" => {
			let custom = input.value("image-url").unwrap_or("").trim();
			let url = if custom.is_empty() { None } else { Some(custom) };
			render_search_panel(url)
		}
		_ => Output::default(),
	}
}

pub fn extract_image_url(text: &str) -> Option<String> {
	for word in text.split_whitespace() {
		let clean = word.trim_matches(|c| c == '<' || c == '>' || c == '(' || c == ')' || c == '"');
		if clean.starts_with("http://") || clean.starts_with("https://") {
			let lower = clean.to_ascii_lowercase();
			if lower.contains(".png")
				|| lower.contains(".jpg")
				|| lower.contains(".jpeg")
				|| lower.contains(".webp")
				|| lower.contains(".gif")
				|| lower.contains("cdn.discordapp.com/attachments/")
				|| lower.contains("media.discordapp.net/attachments/")
			{
				return Some(clean.to_owned());
			}
		}
	}
	None
}

fn encode_url(raw: &str) -> String {
	url::form_urlencoded::byte_serialize(raw.as_bytes()).collect()
}

fn render_search_panel(image_url: Option<&str>) -> Output {
	let mut panel = vec![
		Element::Heading {
			text: "Reverse Image Search".to_owned(),
		},
		Element::Text {
			text: "Быстрый поиск дубликатов, первоисточников и артов в поисковиках картинок."
				.to_owned(),
		},
		Element::Separator,
	];

	if let Some(target) = image_url {
		let encoded = encode_url(target);
		let google = format!("https://lens.google.com/uploadbyurl?url={encoded}");
		let yandex = format!("https://yandex.com/images/search?rpt=imageview&url={encoded}");
		let saucenao = format!("https://saucenao.com/search.php?url={encoded}");
		let tineye = format!("https://tineye.com/search?url={encoded}");
		let ascii2d = format!("https://ascii2d.net/search/url/{encoded}");

		panel.push(Element::Heading {
			text: "Целевое изображение:".to_owned(),
		});
		panel.push(Element::Text {
			text: target.to_owned(),
		});
		panel.push(Element::Separator);
		panel.push(Element::Heading {
			text: "Поисковые системы:".to_owned(),
		});
		panel.push(Element::Text {
			text: format!("🔍 Google Lens:\n{google}"),
		});
		panel.push(Element::Text {
			text: format!("🔍 Yandex Images:\n{yandex}"),
		});
		panel.push(Element::Text {
			text: format!("🎨 SauceNAO (Аниме / Арт):\n{saucenao}"),
		});
		panel.push(Element::Text {
			text: format!("🔍 TinEye:\n{tineye}"),
		});
		panel.push(Element::Text {
			text: format!("🎨 Ascii2D:\n{ascii2d}"),
		});
	} else {
		panel.push(Element::Text {
			text: "В выбранном сообщении не найдена прямая ссылка на изображение. Вы можете вставить URL изображения вручную ниже:".to_owned(),
		});
	}

	panel.push(Element::Separator);
	panel.push(Element::TextInput {
		id: "image-url".to_owned(),
		label: "URL изображения (https://...)".to_owned(),
		value: image_url.unwrap_or("").to_owned(),
	});
	panel.push(Element::Button {
		id: "calc-engines".to_owned(),
		label: "Сформировать ссылки поиска".to_owned(),
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

	#[test]
	fn test_extract_discord_cdn_url() {
		let msg = "Check this art: https://cdn.discordapp.com/attachments/123/456/artwork.png cool!";
		let found = extract_image_url(msg);
		assert_eq!(
			found.as_deref(),
			Some("https://cdn.discordapp.com/attachments/123/456/artwork.png")
		);
	}

	#[test]
	fn test_extract_angle_bracket_url() {
		let msg = "Look at <https://example.com/pic.jpg>";
		let found = extract_image_url(msg);
		assert_eq!(found.as_deref(), Some("https://example.com/pic.jpg"));
	}

	#[test]
	fn test_render_search_engines() {
		let input = Invocation {
			action: "search-image".to_owned(),
			selected_message: Some("https://example.com/test.png".to_owned()),
			..Default::default()
		};
		let output = handle(input);
		assert!(!output.panel.is_empty());
		let has_lens = output.panel.iter().any(|e| match e {
			Element::Text { text } => text.contains("lens.google.com"),
			_ => false,
		});
		assert!(has_lens);
	}
}
