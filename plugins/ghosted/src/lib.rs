/// VenSerein (Ghosted): Ghost ping tracker and notifications.
use serein_extension_sdk::{export, Element, Invocation, Output};

fn handle(inv: Invocation) -> Output {
    if inv.action == "ghost-log" {
        Output {
            panel: vec![
                Element::Text { text: "Ghost Ping Detector".into() },
                Element::Text { text: "Logged ghost pings: 0 in current session.".into() },
            ],
            ..Default::default()
        }
    } else {
        Output::default()
    }
}

export!(handle);
