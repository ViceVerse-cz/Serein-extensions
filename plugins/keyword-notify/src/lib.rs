/// VenSerein (KeywordNotify): Custom alert keywords and notification rules.
use serein_extension_sdk::{export, Element, Invocation, Output};

fn handle(inv: Invocation) -> Output {
    if inv.action == "notify-keywords" {
        Output {
            panel: vec![
                Element::Text { text: "Notification Trigger Keywords".into() },
                Element::TextInput { id: "keywords".into(), label: "Alert Keywords (comma-separated)".into(), value: String::new() },
            ],
            ..Default::default()
        }
    } else {
        Output::default()
    }
}

export!(handle);
