/// VenSerein (CustomStatusTimeouts): Custom status timeout presets.
use serein_extension_sdk::{export, Element, Invocation, Output};

fn handle(inv: Invocation) -> Output {
    if inv.action == "timeouts-panel" {
        Output {
            panel: vec![
                Element::Text { text: "Custom Status Timeout Intervals".into() },
                Element::Text { text: "Available intervals: 15m, 30m, 1h, 4h, 8h, 1d, 3d, 1w".into() },
            ],
            ..Default::default()
        }
    } else {
        Output::default()
    }
}

export!(handle);
