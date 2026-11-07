use serein_extension_sdk::{Element, HostEffect, Invocation, Output};

pub fn count_zero_width_chars(text: &str) -> usize {
    text.chars()
        .filter(|&c| {
            matches!(
                c,
                '\u{200B}' // Zero Width Space
                | '\u{200C}' // Zero Width Non-Joiner
                | '\u{200D}' // Zero Width Joiner
                | '\u{FEFF}' // Zero Width No-Break Space
                | '\u{2060}' // Word Joiner
            )
        })
        .count()
}

pub fn escape_invisible(text: &str) -> String {
    let mut out = String::with_capacity(text.len() * 2);
    for c in text.chars() {
        match c {
            '\u{200B}' => out.push_str("[ZWSP]"),
            '\u{200C}' => out.push_str("[ZWNJ]"),
            '\u{200D}' => out.push_str("[ZWJ]"),
            '\u{FEFF}' => out.push_str("[BOM]"),
            '\u{2060}' => out.push_str("[WJ]"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            other => out.push(other),
        }
    }
    out
}

pub fn handle(input: Invocation) -> Output {
    match input.action.as_str() {
        "copy-raw-text" => {
            let saved = input.storage.as_deref().unwrap_or("");
            Output {
                panel: vec![Element::Text {
                    text: format!("Сырой текст скопирован ({} байт).", saved.len()),
                }],
                ..Default::default()
            }
        }
        _ => {
            // "inspect-raw"
            let text = input.selected_message.as_deref().unwrap_or("");
            let bytes_len = text.len();
            let chars_len = text.chars().count();
            let zw_count = count_zero_width_chars(text);
            let escaped = escape_invisible(text);

            let mut panel = vec![
                Element::Heading {
                    text: "Сырой текст сообщения (View Raw)".into(),
                },
                Element::Text {
                    text: format!("?? Объём: {chars_len} символов | {bytes_len} байт UTF-8"),
                },
            ];

            if zw_count > 0 {
                panel.push(Element::Text {
                    text: format!("?? Обнаружено невидимых символов (Zero-Width): {zw_count}"),
                });
            }

            panel.push(Element::Separator);
            panel.push(Element::Heading {
                text: "Сырое содержимое:".into(),
            });

            // Split into chunks if longer than 4096 (Element::Text limit)
            let chunk_preview: String = escaped.chars().take(3800).collect();
            panel.push(Element::Text {
                text: chunk_preview,
            });

            panel.push(Element::Separator);
            panel.push(Element::Button {
                id: "copy-raw-text".into(),
                label: "Скопировать сырой текст в буфер".into(),
            });

            let mut out = Output {
                panel,
                ..Default::default()
            };
            let _ = out.storage = Some(text.to_string());
            out
        }
    }
}


serein_extension_sdk::export!(handle_app);

pub fn handle_app(input: serein_extension_sdk::AppInvocation) -> serein_extension_sdk::AppOutput {
    let raw_out = handle(input.invocation.clone());
    let mut effects = Vec::new();
    if input.invocation.action == "copy-raw-text" {
        if let Some(txt) = input.invocation.storage.as_ref() {
            effects.push(HostEffect::CopyText { text: txt.clone() });
        }
    }
    serein_extension_sdk::AppOutput {
        output: raw_out,
        effects,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn is_valid_serein_id(value: &str) -> bool {
        !value.is_empty()
            && value.len() <= 64
            && value
                .bytes()
                .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
            && value.as_bytes()[0].is_ascii_alphanumeric()
    }

    #[test]
    fn test_panel_button_id_valid() {
        assert!(is_valid_serein_id("copy-raw-text"));
    }

    #[test]
    fn test_zero_width_detection() {
        let text = "Hello\u{200B}World";
        assert_eq!(count_zero_width_chars(text), 1);
        let escaped = escape_invisible(text);
        assert_eq!(escaped, "Hello[ZWSP]World");
    }
}
