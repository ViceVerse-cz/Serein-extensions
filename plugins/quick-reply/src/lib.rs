use serein_extension_sdk::{Element, Invocation, Output};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Snippet {
    pub title: String,
    pub content: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SnippetStore {
    pub snippets: Vec<Snippet>,
}

impl Default for SnippetStore {
    fn default() -> Self {
        Self {
            snippets: vec![
                Snippet {
                    title: "Приветствие".into(),
                    content: "Привет! Чем могу помочь?".into(),
                },
                Snippet {
                    title: "Благодарность".into(),
                    content: "Большое спасибо за помощь!".into(),
                },
                Snippet {
                    title: "Отошёл (AFK)".into(),
                    content: "Отошёл на некоторое время, скоро вернусь.".into(),
                },
                Snippet {
                    title: "Код / Ошибка".into(),
                    content: "Пришлите, пожалуйста, логи консоли и шаги воспроизведения.".into(),
                },
                Snippet {
                    title: "Всё готово".into(),
                    content: "? Всё готово и успешно проверено.".into(),
                },
            ],
        }
    }
}

fn build_composer_panel(store: &SnippetStore) -> Output {
    if store.snippets.is_empty() {
        return Output {
            panel: vec![Element::Text {
                text: "Список шаблонов пуст. Добавьте шаблоны в настройках плагина.".into(),
            }],
            ..Default::default()
        };
    }

    let options = store
        .snippets
        .iter()
        .map(|s| s.title.clone())
        .collect::<Vec<_>>();
    let default_val = options.first().cloned().unwrap_or_default();

    Output {
        panel: vec![
            Element::Heading {
                text: "Выбор быстрого ответа".into(),
            },
            Element::Select {
                id: "selected-title".into(),
                label: "Шаблон".into(),
                options,
                value: default_val,
            },
            Element::Button {
                id: "apply-snippet".into(),
                label: "Вставить в драфт".into(),
            },
        ],
        ..Default::default()
    }
}

fn build_settings_panel(store: &SnippetStore, notice: Option<&str>) -> Output {
    let mut panel = vec![
        Element::Heading {
            text: "Управление быстрыми ответами".into(),
        },
        Element::Text {
            text: format!("Всего шаблонов: {}", store.snippets.len()),
        },
    ];

    if let Some(msg) = notice {
        panel.push(Element::Separator);
        panel.push(Element::Text { text: msg.into() });
    }

    panel.push(Element::Separator);
    panel.push(Element::Heading {
        text: "Добавить новый шаблон".into(),
    });
    panel.push(Element::TextInput {
        id: "new-title".into(),
        label: "Название (например, 'Согласен')".into(),
        value: String::new(),
    });
    panel.push(Element::TextInput {
        id: "new-content".into(),
        label: "Текст ответа".into(),
        value: String::new(),
    });
    panel.push(Element::Button {
        id: "add-snippet".into(),
        label: "Сохранить шаблон".into(),
    });
    panel.push(Element::Button {
        id: "reset-defaults".into(),
        label: "Сбросить к стандартным".into(),
    });

    panel.push(Element::Separator);
    panel.push(Element::Heading {
        text: "Текущие шаблоны:".into(),
    });
    for s in &store.snippets {
        panel.push(Element::Text {
            text: format!("• **{}**: {}", s.title, s.content),
        });
    }

    Output {
        panel,
        ..Default::default()
    }
}

pub fn handle(input: Invocation) -> Output {
    let mut store = input
        .storage_json::<SnippetStore>()
        .ok()
        .flatten()
        .unwrap_or_default();

    match input.action.as_str() {
        "show-snippets" => build_composer_panel(&store),
        "apply-snippet" => {
            let title = input.value("selected-title").unwrap_or("");
            if let Some(found) = store.snippets.iter().find(|s| s.title == title) {
                Output {
                    replacement: Some(found.content.clone()),
                    panel: vec![Element::Text {
                        text: format!("Вставлен шаблон: \"{}\"", found.title),
                    }],
                    ..Default::default()
                }
            } else {
                build_composer_panel(&store)
            }
        }
        "open-settings" => build_settings_panel(&store, None),
        "add-snippet" => {
            let title = input.value("new-title").unwrap_or("").trim().to_string();
            let content = input.value("new-content").unwrap_or("").trim().to_string();

            if title.is_empty() || content.is_empty() {
                return build_settings_panel(&store, Some("Ошибка: название и текст не могут быть пустыми!"));
            }

            store.snippets.retain(|s| s.title != title);
            store.snippets.push(Snippet { title, content });

            let mut out = build_settings_panel(&store, Some("Шаблон успешно сохранён!"));
            let _ = out.set_storage_json(&store);
            out
        }
        "reset-defaults" => {
            let defaults = SnippetStore::default();
            let mut out = build_settings_panel(&defaults, Some("Шаблоны сброшены к стандартным."));
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
        let store = SnippetStore::default();
        let composer_out = build_composer_panel(&store);
        for element in composer_out.panel {
            match element {
                Element::Button { id, label }
                | Element::Select { id, label, .. }
                | Element::TextInput { id, label, .. } => {
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

        let settings_out = build_settings_panel(&store, None);
        for element in settings_out.panel {
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
    fn test_snippet_store_defaults() {
        let store = SnippetStore::default();
        assert!(!store.snippets.is_empty());
        let afk = store.snippets.iter().find(|s| s.title.contains("AFK"));
        assert!(afk.is_some());
    }

    #[test]
    fn test_add_snippet() {
        let mut store = SnippetStore::default();
        let initial_count = store.snippets.len();
        store.snippets.push(Snippet {
            title: "Test".into(),
            content: "Hello Test".into(),
        });
        assert_eq!(store.snippets.len(), initial_count + 1);
    }
}
