use serein_extension_sdk::{Element, Invocation, Output};

/// Parses markdown masked links `[label](url)` and replaces them with plain `url`
pub fn unmask_markdown_links(text: &str) -> (String, usize) {
    let mut result = String::with_capacity(text.len());
    let mut unmasked_count = 0;
    let mut last_idx = 0;

    let bytes = text.as_bytes();
    let len = text.len();

    let mut i = 0;
    while i < len {
        if bytes[i] == b'[' {
            // Find closing ]
            if let Some(close_bracket) = text[i..].find(']') {
                let label_end = i + close_bracket;
                let label = &text[i + 1..label_end];

                // Check if immediately followed by (
                if label_end + 1 < len && bytes[label_end + 1] == b'(' {
                    let url_start = label_end + 2;
                    if let Some(close_paren) = text[url_start..].find(')') {
                        let url_end = url_start + close_paren;
                        let url = &text[url_start..url_end];

                        if url.starts_with("http://") || url.starts_with("https://") {
                            result.push_str(&text[last_idx..i]);
                            // If label is different from url, expose both: url (label) or just url
                            if label.is_empty() || label == url {
                                result.push_str(url);
                            } else {
                                result.push_str(&format!("{url} ({label})"));
                            }
                            unmasked_count += 1;
                            last_idx = url_end + 1;
                            i = last_idx;
                            continue;
                        }
                    }
                }
            }
        }
        i += 1;
    }

    if last_idx < len {
        result.push_str(&text[last_idx..]);
    }

    (result, unmasked_count)
}

pub fn handle(input: Invocation) -> Output {
    if input.action != "unmask-links" {
        return Output::default();
    }

    let draft = input.composer.as_deref().unwrap_or("");
    let (unmasked, count) = unmask_markdown_links(draft);

    if count > 0 {
        Output {
            replacement: Some(unmasked),
            panel: vec![Element::Text {
                text: format!("Размаскировано скрытых ссылок: {count}."),
            }],
            ..Default::default()
        }
    } else {
        Output {
            panel: vec![Element::Text {
                text: "В драфте не найдено маскированных markdown-ссылок вида [текст](url).".into(),
            }],
            ..Default::default()
        }
    }
}

serein_extension_sdk::export!(handle);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unmask_phishing_link() {
        let input = "Check out [https://google.com](https://evil-phishing.com/login) now!";
        let (output, count) = unmask_markdown_links(input);
        assert_eq!(count, 1);
        assert_eq!(
            output,
            "Check out https://evil-phishing.com/login (https://google.com) now!"
        );
    }

    #[test]
    fn test_unmask_identical_label() {
        let input = "[https://example.com](https://example.com)";
        let (output, count) = unmask_markdown_links(input);
        assert_eq!(count, 1);
        assert_eq!(output, "https://example.com");
    }
}
