/// VenSerein (BetterBanReasons): Ban reason presets and moderation templates.
use serein_extension_sdk::{export, Element, Invocation, Output};

fn handle(inv: Invocation) -> Output {
    match inv.action.as_str() {
        "ban-template" => Output {
            replacement: Some("Rule 1: Spamming / Promotional Links".into()),
            ..Default::default()
        },
        "manage-reasons" => Output {
            panel: vec![
                Element::Text { text: "Moderation Reason Presets".into() },
                Element::Text { text: "Presets: Spam, Harassment, Inappropriate Content, Raid".into() },
            ],
            ..Default::default()
        },
        _ => Output::default(),
    }
}

export!(handle);
