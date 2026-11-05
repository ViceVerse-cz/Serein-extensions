/// VenSerein (SilenceUsers): Client-side chat muting per user.
use serein_extension_sdk::{export, Element, Invocation, Output};

fn handle(inv: Invocation) -> Output {
    if inv.action == "silence-list" {
        Output {
            panel: vec![
                Element::Text { text: "Silenced Users (Client-Side)".into() },
                Element::Text { text: "Muted participants will have their messages hidden locally.".into() },
            ],
            ..Default::default()
        }
    } else {
        Output::default()
    }
}

export!(handle);
