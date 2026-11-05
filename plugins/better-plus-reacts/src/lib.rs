/// VenSerein (BetterPlusReacts): Parse +N :emoji: syntax to react to earlier messages.
use serein_extension_sdk::{export, Element, Invocation, Output};

fn handle(inv: Invocation) -> Output {
    if inv.action != "parse-plus-react" {
        return Output::default();
    }
    let text = inv.composer.as_deref().unwrap_or("");
    let trimmed = text.trim();
    if trimmed.starts_with('+') {
        let count = trimmed.chars().take_while(|&c| c == '+').count();
        let emoji = trimmed.trim_start_matches('+').trim();
        Output {
            panel: vec![
                Element::Text { text: format!("Reaction queued: {} on message -{}", emoji, count) },
            ],
            replacement: Some(String::new()),
            ..Default::default()
        }
    } else {
        Output::default()
    }
}

export!(handle);
