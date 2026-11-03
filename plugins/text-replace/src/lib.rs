use serein_extension_sdk::{Element, Invocation, Output};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct MacroRule {
    pub pattern: String,
    pub replacement: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct TextReplaceStore {
    pub rules: Vec<MacroRule>,
}

impl Default for TextReplaceStore {
    fn default() -> Self {
        Self {
            rules: vec![
                MacroRule {
                    pattern: "(shrug)".into(),
                    replacement: r"¯\_(ツ)_/¯".into(),
                },
                MacroRule {
                    pattern: "(tableflip)".into(),
                    replacement: "(╯°□°)╯︵ ┻━┻".into(),
                },
                MacroRule {
                    pattern: "(unflip)".into(),
                    replacement: "┬─┬ノ( º _ ºノ)".into(),
                },
                MacroRule {
                    pattern: "(lenny)".into(),
                    replacement: "( ͡° ͜ʖ ͡°)".into(),
                },
                MacroRule {
                    pattern: "(disapprove)".into(),
                    replacement: "ಠ_ಠ".into(),
                },
                MacroRule {
                    pattern: "(gimme)".into(),
                    replacement: "༼ つ ◕_◕ ༽つ".into(),
                },
                MacroRule {
                    pattern: "(heart)".into(),
                    replacement: "❤️".into(),
                },
                MacroRule {
                    pattern: "(fire)".into(),
                    replacement: "🔥".into(),
                },
            ],
        }
    }
}

pub fn apply_macros(text: &str, rules: &[MacroRule]) -> (String, usize) {
    let mut current = text.to_string();
    let mut replaced_count = 0;

    for rule in rules {
        if rule.pattern.is_empty() {
            continue;
        }
        let matches = current.matches(&rule.pattern).count();
        if matches > 0 {
            current = current.replace(&rule.pattern, &rule.replacement);
            replaced_count += matches;
        }
    }

    (current, replaced_count)
}

fn build_settings_panel(store: &TextReplaceStore, notice: Option<&str>) -> Output {
    let mut panel = vec![
        Element::Heading {
            text: "Text Replace & Kaomoji Macros".into(),
        },
        Element::Text {
            text: format!("Всего активных правил: {}", store.rules.len()),
        },
    ];

    if let Some(msg) = notice {
        panel.push(Element::Separator);
        panel.push(Element::Text { text: msg.into() });
    }

    panel.push(Element::Separator);
    panel.push(Element::Heading {
        text: "Добавить новое правило".into(),
    });
    panel.push(Element::TextInput {
        id: "new-pattern".into(),
        label: "Шаблон (например, :bye: или (wave))".into(),
        value: String::new(),
    });
    panel.push(Element::TextInput {
        id: "new-replacement".into(),
        label: "Замена (текст или эмодзи)".into(),
        value: String::new(),
    });
    panel.push(Element::Button {
        id: "add-macro".into(),
        label: "Добавить макрос".into(),
    });
    panel.push(Element::Button {
        id: "reset-defaults".into(),
        label: "Восстановить правила по умолчанию".into(),
    });

    panel.push(Element::Separator);
    panel.push(Element::Heading {
        text: "Список правил:".into(),
    });
    for rule in &store.rules {
        panel.push(Element::Text {
            text: format!("• \"{}\"  ➔  \"{}\"", rule.pattern, rule.replacement),
        });
    }

    Output {
        panel,
        ..Default::default()
    }
}

pub fn handle(input: Invocation) -> Output {
    let mut store = input
        .storage_json::<TextReplaceStore>()
        .ok()
        .flatten()
        .unwrap_or_default();

    match input.action.as_str() {
        "expand-macros" => {
            let draft = input.composer.as_deref().unwrap_or("");
            let (replaced, count) = apply_macros(draft, &store.rules);
            if count > 0 {
                Output {
                    replacement: Some(replaced),
                    panel: vec![Element::Text {
                        text: format!("Развернуто макросов: {count}"),
                    }],
                    ..Default::default()
                }
            } else {
                Output {
                    panel: vec![Element::Text {
                        text: "В драфте не найдено совпадений с шаблонами макросов.".into(),
                    }],
                    ..Default::default()
                }
            }
        }
        "open-settings" => build_settings_panel(&store, None),
        "add-macro" => {
            let pattern = input.value("new-pattern").unwrap_or("").trim().to_string();
            let replacement = input.value("new-replacement").unwrap_or("").trim().to_string();

            if pattern.is_empty() || replacement.is_empty() {
                return build_settings_panel(&store, Some("Ошибка: шаблон и замена не могут быть пустыми!"));
            }

            // Remove existing duplicate if any
            store.rules.retain(|r| r.pattern != pattern);
            if store.rules.len() >= 100 {
                store.rules.remove(0);
            }
            store.rules.push(MacroRule { pattern, replacement });

            let mut out = build_settings_panel(&store, Some("Новое правило успешно добавлено!"));
            let _ = out.set_storage_json(&store);
            out
        }
        "reset-defaults" => {
            let defaults = TextReplaceStore::default();
            let mut out = build_settings_panel(&defaults, Some("Правила сброшены к стандартным значениям."));
            let _ = out.set_storage_json(&defaults);
            out
        }
        _ => Output::default(),
    }
}

serein_extension_sdk::export!(handle);

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
    fn test_panel_element_ids_valid() {
        let store = TextReplaceStore::default();
        let out = build_settings_panel(&store, None);
        for element in out.panel {
            match element {
                Element::Button { id, label } | Element::TextInput { id, label, .. } => {
                    assert!(
                        is_valid_serein_id(&id),
                        "Element id '{}' does not satisfy Serein valid_id",
                        id
                    );
                    assert!(!label.is_empty() && label.len() <= 128);
                }
                _ => {}
            }
        }
    }

    #[test]
    fn test_apply_default_macros() {
        let store = TextReplaceStore::default();
        let input = "Well (shrug) let's see!";
        let (output, count) = apply_macros(input, &store.rules);
        assert_eq!(count, 1);
        assert_eq!(output, r"Well ¯\_(ツ)_/¯ let's see!");
    }

    #[test]
    fn test_multiple_replacements() {
        let store = TextReplaceStore::default();
        let input = "(tableflip) and then (unflip)";
        let (output, count) = apply_macros(input, &store.rules);
        assert_eq!(count, 2);
        assert_eq!(output, "(╯°□°)╯︵ ┻━┻ and then ┬─┬ノ( º _ ºノ)");
    }

    #[test]
    fn test_emoji_macro_replacements() {
        let store = TextReplaceStore::default();
        let input = "Love is (heart) and passion is (fire)!";
        let (output, count) = apply_macros(input, &store.rules);
        assert_eq!(count, 2);
        assert_eq!(output, "Love is ❤️ and passion is 🔥!");
    }
}
