/// VenSerein (UrlHighlighter): Highlight and categorize URLs in chat.
use serein_extension_sdk::{export, Invocation, Output, Element};

fn handle(inv: Invocation) -> Output {
    if inv.action == "toggle" {
        let text = inv.selected_message.as_deref().unwrap_or(
            inv.composer.as_deref().unwrap_or("")
        );
        let found = find_urls(text);
        if found.is_empty() {
            return Output {
                panel: vec![Element::Text { text: "No URLs found in selected text.".into() }],
                ..Default::default()
            };
        }
        let items: Vec<Element> = found.iter()
            .map(|(url, cat)| Element::Text { text: format!("[{}] {}", cat, url) })
            .collect();
        Output { panel: items, ..Default::default() }
    } else {
        Output::default()
    }
}

fn find_urls(text: &str) -> Vec<(String, &'static str)> {
    text.split_whitespace()
        .filter(|w| w.starts_with("http://") || w.starts_with("https://"))
        .map(|url| {
            let cat = categorize(url);
            (url.to_owned(), cat)
        })
        .collect()
}

fn categorize(url: &str) -> &'static str {
    if url.contains("bit.ly") || url.contains("t.co") || url.contains("tinyurl") {
        "shortened"
    } else if url.contains("tracking") || url.contains("utm_") || url.contains("fbclid") {
        "tracking"
    } else {
        "safe"
    }
}

export!(handle);
