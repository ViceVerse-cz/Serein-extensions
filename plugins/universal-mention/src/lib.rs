/// VenSerein (UniversalMention): Fuzzy lookup for user, role, and channel mentions.
use serein_extension_sdk::{export, Invocation, Output};

fn handle(inv: Invocation) -> Output {
    if inv.action != "fuzzy-mention" {
        return Output::default();
    }
    let query = inv.composer.as_deref().unwrap_or("").trim();
    if query.starts_with('@') {
        Output {
            replacement: Some(format!("<{}>", query)),
            ..Default::default()
        }
    } else {
        Output::default()
    }
}

export!(handle);
