use serein_extension_sdk::{AppInvocation, AppOutput, Element, Output};

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

pub fn append_mention_to_draft(draft: &str, author_id: &str) -> String {
    let tag = format!("<@{author_id}>");
    if draft.is_empty() {
        format!("{tag} ")
    } else {
        format!("{draft} {tag} ")
    }
}

pub fn handle(input: AppInvocation) -> AppOutput {
    let inv = &input.invocation;

    if inv.action != "mention-author" {
        return AppOutput::default();
    }

    let msg_text = inv.selected_message.as_deref().unwrap_or("");

    // Find author ID from timeline or message content
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
        .or_else(|| extract_mentioned_user_id(msg_text).map(|s| s.to_string()));

    if let Some(uid) = author_id {
        let current_draft = inv.composer.as_deref().unwrap_or("");
        let new_draft = append_mention_to_draft(current_draft, &uid);

        AppOutput {
            output: Output {
                replacement: Some(new_draft),
                panel: vec![Element::Text {
                    text: format!("Автор <@{uid}> добавлен в драфт сообщения."),
                }],
                ..Default::default()
            },
            ..Default::default()
        }
    } else {
        AppOutput {
            output: Output {
                panel: vec![Element::Text {
                    text: "Не удалось определить ID автора для выбранного сообщения.".into(),
                }],
                ..Default::default()
            },
            ..Default::default()
        }
    }
}

serein_extension_sdk::export!(handle);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_append_mention_empty_draft() {
        let res = append_mention_to_draft("", "123456789");
        assert_eq!(res, "<@123456789> ");
    }

    #[test]
    fn test_append_mention_existing_draft() {
        let res = append_mention_to_draft("Hello", "123456789");
        assert_eq!(res, "Hello <@123456789> ");
    }

    #[test]
    fn test_extract_mentioned_user_id() {
        let text = "Check this <@987654321>";
        let id = extract_mentioned_user_id(text).unwrap();
        assert_eq!(id, "987654321");
    }
}
