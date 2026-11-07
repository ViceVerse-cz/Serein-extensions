/// VenSerein (BetterActivities): Activity launcher and status details inspector.
use serein_extension_sdk::{export, Element, Invocation, Output};

fn handle(inv: Invocation) -> Output {
    if inv.action == "activity-panel" {
        Output {
            panel: vec![
                Element::Text { text: "Active Activities & Launchers".into() },
                Element::Text { text: "Manage embedded Discord Activities, Party games, and Rich Presence sessions.".into() },
            ],
            ..Default::default()
        }
    } else {
        Output::default()
    }
}

export!(handle);
