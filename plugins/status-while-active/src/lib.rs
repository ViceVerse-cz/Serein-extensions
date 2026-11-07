/// VenSerein (StatusWhileActive): Dynamic presence rules while playing or streaming.
use serein_extension_sdk::{export, Element, Invocation, Output};

fn handle(inv: Invocation) -> Output {
    if inv.action == "active-rules" {
        Output {
            panel: vec![
                Element::Text { text: "Activity-Based Status Automations".into() },
                Element::Text { text: "Automatically toggle DND when fullscreen games or stream mode begins.".into() },
            ],
            ..Default::default()
        }
    } else {
        Output::default()
    }
}

export!(handle);
