use serein_extension_sdk::{AppInvocation, AppOutput, Element, HostEffect, Output};

pub fn build_user_url(user_id: &str) -> String {
    format!("https://discord.com/users/{user_id}")
}

/// Extract user ID mentions from message content: <@123456789> or <@!123456789>
pub fn extract_mentioned_user_id(content: &str) -> Option<&str> {
    let start_tag = "<@";
    let start = content.find(start_tag)?;
    let rest = &content[start + start_tag.len()..];
    let rest = rest.strip_prefix('!').unwrap_or(rest);
    let end = rest.find('>')?;
    let id_candidate = &rest[..end];
    if id_candidate.bytes().all(|b| b.is_ascii_digit()) && !id_candidate.is_empty() {
        Some(id_candidate)
    } else {
        None
    }
}

fn build_panel(current_id: &str, result_notice: Option<&str>) -> Output {
    let mut panel = vec![
        Element::Heading {
            text: "User Profile URL Generator".into(),
        },
        Element::Text {
            text: "Введите ID пользователя Discord (Snowflake):".into(),
        },
        Element::TextInput {
            id: "user-id-input".into(),
            label: "User ID".into(),
            value: current_id.into(),
        },
        Element::Button {
            id: "generate-url".into(),
            label: "Скопировать ссылку профиля".into(),
        },
    ];

    if let Some(msg) = result_notice {
        panel.push(Element::Separator);
        panel.push(Element::Text { text: msg.into() });
    }

    Output {
        panel,
        ..Default::default()
    }
}

pub fn handle(input: AppInvocation) -> AppOutput {
    let inv = &input.invocation;

    match inv.action.as_str() {
        "copy-author-url" => {
            let msg_text = inv.selected_message.as_deref().unwrap_or("");

            // 1. Check if the message matches an item in timeline to find author id
            let author_id = input
                .app
                .as_ref()
                .and_then(|a| a.timeline.as_ref())
                .and_then(|t| {
                    t.messages
                        .iter()
                        .find(|m| m.content == msg_text)
                        .map(|m| m.author.id.clone())
                })
                // 2. Or check for mentioned users in the message text
                .or_else(|| extract_mentioned_user_id(msg_text).map(|s| s.to_string()))
                // 3. Or check members snapshot
                .or_else(|| {
                    input
                        .app
                        .as_ref()
                        .and_then(|a| a.members.as_ref())
                        .and_then(|m| m.items.first().map(|u| u.id.clone()))
                });

            if let Some(uid) = author_id {
                let url = build_user_url(&uid);
                AppOutput {
                    output: Output {
                        panel: vec![
                            Element::Heading {
                                text: "Ссылка профиля скопирована".into(),
                            },
                            Element::Text { text: url.clone() },
                        ],
                        ..Default::default()
                    },
                    effects: vec![HostEffect::CopyText { text: url }],
                }
            } else {
                AppOutput {
                    output: Output {
                        panel: vec![Element::Text {
                            text: "Не удалось автоматически определить ID пользователя для этого сообщения.".into(),
                        }],
                        ..Default::default()
                    },
                    ..Default::default()
                }
            }
        }
        "open-tool" => AppOutput {
            output: build_panel("", None),
            ..Default::default()
        },
        "generate-url" => {
            let user_id = inv.value("user-id-input").unwrap_or("").trim();
            if user_id.is_empty() || !user_id.bytes().all(|b| b.is_ascii_digit()) {
                return AppOutput {
                    output: build_panel(user_id, Some("Ошибка: введите корректный цифровой Discord ID.")),
                    ..Default::default()
                };
            }

            let url = build_user_url(user_id);
            AppOutput {
                output: build_panel(user_id, Some(&format!("Скопировано в буфер: {url}"))),
                effects: vec![HostEffect::CopyText { text: url }],
            }
        }
        _ => AppOutput::default(),
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
        let out = build_panel("1234567890", None);
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
    fn test_build_user_url() {
        let url = build_user_url("38383838383838");
        assert_eq!(url, "https://discord.com/users/38383838383838");
    }

    #[test]
    fn test_extract_mentioned_user_id() {
        let text = "Hey <@!987654321012345678> what's up?";
        let extracted = extract_mentioned_user_id(text).unwrap();
        assert_eq!(extracted, "987654321012345678");
    }
}
