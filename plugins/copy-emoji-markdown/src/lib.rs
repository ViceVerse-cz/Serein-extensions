use serein_extension_sdk::{AppInvocation, AppOutput, Element, HostEffect, Output};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CustomEmoji {
    pub animated: bool,
    pub name: String,
    pub id: String,
}

impl CustomEmoji {
    pub fn markdown(&self) -> String {
        let prefix = if self.animated { "a" } else { "" };
        format!("<{prefix}:{}:{}>", self.name, self.id)
    }

    pub fn cdn_url(&self) -> String {
        let ext = if self.animated { "gif" } else { "png" };
        format!("https://cdn.discordapp.com/emojis/{}.{ext}?size=128", self.id)
    }
}

/// Parses custom emojis: <:name:id> or <a:name:id>
pub fn extract_custom_emojis(content: &str) -> Vec<CustomEmoji> {
    let mut emojis = Vec::new();
    let mut idx = 0;

    while let Some(start) = content[idx..].find('<') {
        let actual_start = idx + start;
        let rest = &content[actual_start + 1..];

        let (animated, tag_rest) = if let Some(r) = rest.strip_prefix("a:") {
            (true, r)
        } else if let Some(r) = rest.strip_prefix(':') {
            (false, r)
        } else {
            idx = actual_start + 1;
            continue;
        };

        if let Some((name, after_name)) = tag_rest.split_once(':') {
            if let Some(end) = after_name.find('>') {
                let id = &after_name[..end];
                if id.bytes().all(|b| b.is_ascii_digit()) && !id.is_empty() && !name.is_empty() {
                    emojis.push(CustomEmoji {
                        animated,
                        name: name.to_string(),
                        id: id.to_string(),
                    });
                    idx = actual_start + 1 + (tag_rest.len() - after_name.len()) + end + 1;
                    continue;
                }
            }
        }

        idx = actual_start + 1;
    }

    emojis
}

pub fn handle(input: AppInvocation) -> AppOutput {
    let inv = &input.invocation;

    match inv.action.as_str() {
        "copy-emoji-text" => {
            let saved = inv.storage.as_deref().unwrap_or("");
            AppOutput {
                output: Output {
                    panel: vec![Element::Text {
                        text: "Эмодзи успешно скопированы в буфер обмена.".into(),
                    }],
                    ..Default::default()
                },
                effects: vec![HostEffect::CopyText {
                    text: saved.to_string(),
                }],
            }
        }
        _ => {
            // "extract-emojis"
            let msg = inv.selected_message.as_deref().unwrap_or("");
            let emojis = extract_custom_emojis(msg);

            if emojis.is_empty() {
                return AppOutput {
                    output: Output {
                        panel: vec![Element::Text {
                            text: "В выбранном сообщении не найдено кастомных Discord эмодзи.".into(),
                        }],
                        ..Default::default()
                    },
                    ..Default::default()
                };
            }

            let mut panel = vec![
                Element::Heading {
                    text: format!("Найдено кастомных эмодзи: {}", emojis.len()),
                },
                Element::Separator,
            ];

            let mut all_markdown = Vec::new();
            for e in &emojis {
                let md = e.markdown();
                let cdn = e.cdn_url();
                panel.push(Element::Text {
                    text: format!("• **:{}**: `{}` (Анимированный: {})", e.name, md, e.animated),
                });
                panel.push(Element::Text {
                    text: format!("  CDN: {cdn}"),
                });
                all_markdown.push(md);
            }

            panel.push(Element::Separator);
            panel.push(Element::Button {
                id: "copy-emoji-text".into(),
                label: "Скопировать все эмодзи".into(),
            });

            let combined = all_markdown.join(" ");
            let mut out = Output {
                panel,
                ..Default::default()
            };
            out.storage = Some(combined);

            AppOutput {
                output: out,
                ..Default::default()
            }
        }
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
    fn test_panel_button_id_valid() {
        assert!(is_valid_serein_id("copy-emoji-text"));
    }

    #[test]
    fn test_extract_emojis() {
        let msg = "Hello <:pepe:1234567890> and <a:party_blob:987654321> test";
        let list = extract_custom_emojis(msg);
        assert_eq!(list.len(), 2);
        assert_eq!(list[0].name, "pepe");
        assert!(!list[0].animated);
        assert_eq!(list[1].name, "party_blob");
        assert!(list[1].animated);
    }
}
