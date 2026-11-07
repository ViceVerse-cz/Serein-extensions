use serein_extension_sdk::{export, Element, Invocation, Output};
use url::form_urlencoded;

fn detect_script(text: &str) -> (&'static str, &'static str) {
	let mut cyrillic = 0;
	let mut cjk = 0;
	let mut latin = 0;

	for c in text.chars() {
		match c {
			'\u{0400}'..='\u{04FF}' => cyrillic += 1,
			'\u{4E00}'..='\u{9FFF}' | '\u{3040}'..='\u{30FF}' | '\u{AC00}'..='\u{D7AF}' => {
				cjk += 1
			}
			'a'..='z' | 'A'..='Z' => latin += 1,
			_ => {}
		}
	}

	if cyrillic > latin && cyrillic > cjk {
		("Cyrillic (RU/UK/BE)", "en")
	} else if cjk > latin {
		("East Asian (CJK)", "en")
	} else {
		("Latin (EN/ES/FR/DE)", "ru")
	}
}

pub fn handle(input: Invocation) -> Output {
	let text = input
		.selected_message
		.as_deref()
		.or(input.composer.as_deref())
		.unwrap_or("")
		.trim();

	if text.is_empty() {
		return Output {
			panel: vec![
				Element::Heading {
					text: "Translate Message".into(),
				},
				Element::Text {
					text: "No message selected or composer text found to translate.".into(),
				},
			],
			..Default::default()
		};
	}

	let (script_name, target_lang) = detect_script(text);
	let encoded: String = form_urlencoded::byte_serialize(text.as_bytes()).collect();

	let google_url = format!(
		"https://translate.google.com/?sl=auto&tl={target_lang}&text={encoded}&op=translate"
	);
	let deepl_url = format!("https://www.deepl.com/translator#auto/{target_lang}/{encoded}");
	let yandex_url = format!(
		"https://translate.yandex.com/?source_lang=auto&target_lang={target_lang}&text={encoded}"
	);

	let quote_preview = if text.chars().count() > 80 {
		let prefix: String = text.chars().take(77).collect();
		format!("{prefix}…")
	} else {
		text.to_string()
	};

	let replacement_markdown = format!(
		"> {}\n\n🌐 **Translation ({target_lang}):** [Google Translate]({google_url}) • [DeepL]({deepl_url})",
		quote_preview.replace('\n', "\n> ")
	);

	let mut panel = vec![
		Element::Heading {
			text: "Translate Message".into(),
		},
		Element::Text {
			text: format!("Detected script: {script_name} → Target: {target_lang}"),
		},
		Element::Text {
			text: format!(
				"Original text ({} chars):\n\"{quote_preview}\"",
				text.chars().count()
			),
		},
		Element::Text {
			text: format!(
				"• Google Translate: {google_url}\n• DeepL: {deepl_url}\n• Yandex Translate: {yandex_url}"
			),
		},
	];

	if input.action == "insert-links" {
		Output {
			replacement: Some(replacement_markdown),
			panel,
			..Default::default()
		}
	} else {
		panel.push(Element::Button {
			id: "insert-links".into(),
			label: "Insert Translation Links to Composer".into(),
		});

		Output {
			panel,
			..Default::default()
		}
	}
}

export!(handle);

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn detects_cyrillic_and_targets_english() {
		let (script, target) = detect_script("Привет, мир!");
		assert!(script.contains("Cyrillic"));
		assert_eq!(target, "en");
	}

	#[test]
	fn detects_latin_and_targets_russian() {
		let (script, target) = detect_script("Hello, world! How are you doing today?");
		assert!(script.contains("Latin"));
		assert_eq!(target, "ru");
	}

	#[test]
	fn generates_translation_links_and_replacement() {
		let input = Invocation {
			action: "insert-links".into(),
			selected_message: Some("Hello world".into()),
			..Default::default()
		};

		let output = handle(input);
		assert!(output.replacement.is_some());
		let repl = output.replacement.unwrap();
		assert!(repl.contains("https://translate.google.com"));
		assert!(repl.contains("https://www.deepl.com"));
	}
}
