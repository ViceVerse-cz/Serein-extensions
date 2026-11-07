use serein_extension_sdk::{Element, Invocation, Output};

#[derive(Debug, PartialEq, Eq)]
pub struct TextStats {
    pub chars_unicode: usize,
    pub chars_utf16: usize,
    pub bytes: usize,
    pub words: usize,
    pub lines: usize,
    pub reading_time_sec: u64,
}

pub fn analyze_text(text: &str) -> TextStats {
    let chars_unicode = text.chars().count();
    let chars_utf16 = text.encode_utf16().count();
    let bytes = text.len();
    let words = text.split_whitespace().count();
    let lines = if text.is_empty() {
        0
    } else {
        text.lines().count().max(1)
    };

    // Average reading speed: 200 words / minute -> 3.33 words / sec
    let reading_time_sec = if words == 0 {
        0
    } else {
        ((words as f64 / 200.0) * 60.0).ceil() as u64
    };

    TextStats {
        chars_unicode,
        chars_utf16,
        bytes,
        words,
        lines,
        reading_time_sec,
    }
}

pub fn build_stats_panel(title: &str, text: &str) -> Vec<Element> {
    if text.is_empty() {
        return vec![
            Element::Heading {
                text: title.into(),
            },
            Element::Text {
                text: "Текст пуст (0 символов).".into(),
            },
        ];
    }

    let stats = analyze_text(text);

    let standard_limit = 2000;
    let nitro_limit = 4000;

    let standard_pct = (stats.chars_utf16 as f64 / standard_limit as f64) * 100.0;
    let nitro_pct = (stats.chars_utf16 as f64 / nitro_limit as f64) * 100.0;

    let limit_status = if stats.chars_utf16 > nitro_limit {
        format!(
            "?? Превышен лимит Nitro ({}/{} - {:.1}%)!",
            stats.chars_utf16, nitro_limit, nitro_pct
        )
    } else if stats.chars_utf16 > standard_limit {
        format!(
            "?? Требуется Nitro ({} / 2000 - {:.1}%)",
            stats.chars_utf16, standard_pct
        )
    } else {
        format!(
            "? Стандартный лимит: {} / 2000 ({:.1}%)",
            stats.chars_utf16, standard_pct
        )
    };

    vec![
        Element::Heading {
            text: title.into(),
        },
        Element::Separator,
        Element::Text {
            text: format!("?? Статус лимита: {limit_status}"),
        },
        Element::Text {
            text: format!(
                "?? Символов: {} (UTF-16: {}, байт: {})",
                stats.chars_unicode, stats.chars_utf16, stats.bytes
            ),
        },
        Element::Text {
            text: format!("?? Слов: {} | Строк: {}", stats.words, stats.lines),
        },
        Element::Text {
            text: format!("?? Примерное время чтения: ~{} сек.", stats.reading_time_sec),
        },
    ]
}

pub fn handle(input: Invocation) -> Output {
    match input.action.as_str() {
        "inspect-draft" => {
            let text = input.composer.as_deref().unwrap_or("");
            Output {
                panel: build_stats_panel("Статистика драфта", text),
                ..Default::default()
            }
        }
        "inspect-message" => {
            let text = input.selected_message.as_deref().unwrap_or("");
            Output {
                panel: build_stats_panel("Статистика сообщения", text),
                ..Default::default()
            }
        }
        _ => Output::default(),
    }
}

serein_extension_sdk::export!(handle);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_analysis() {
        let stats = analyze_text("");
        assert_eq!(stats.chars_unicode, 0);
        assert_eq!(stats.words, 0);
        assert_eq!(stats.lines, 0);
    }

    #[test]
    fn test_unicode_and_words() {
        let text = "Привет, мир!\nHello world!";
        let stats = analyze_text(text);
        assert_eq!(stats.words, 4);
        assert_eq!(stats.lines, 2);
        assert_eq!(stats.chars_unicode, 25);
    }

    #[test]
    fn test_reading_time() {
        let text = "word ".repeat(100);
        let stats = analyze_text(&text);
        assert_eq!(stats.words, 100);
        assert_eq!(stats.reading_time_sec, 30);
    }
}
