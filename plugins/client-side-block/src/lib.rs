/// VenSerein (ClientSideBlock): Pure local client-side blocking.
use serein_extension_sdk::{export, Element, Invocation, Output};

fn handle(inv: Invocation) -> Output {
    if inv.action == "cs-block-panel" {
        Output {
            panel: vec![
                Element::Text { text: "Local Blocklist (No Gateway Changes)".into() },
                Element::Text { text: "Muted locally: 0 users".into() },
            ],
            ..Default::default()
        }
    } else {
        Output::default()
    }
}

export!(handle);
