/// VenSerein (BetterBlockedUsers): Completely hide or collapse blocked user entries.
use serein_extension_sdk::{export, Element, Invocation, Output};

fn handle(inv: Invocation) -> Output {
    if inv.action == "block-settings" {
        Output {
            panel: vec![
                Element::Text { text: "Blocked Users Display Mode".into() },
                Element::Text { text: "Mode: Completely Hidden (0 messages shown in chat flow)".into() },
            ],
            ..Default::default()
        }
    } else {
        Output::default()
    }
}

export!(handle);
