/// VenSerein (StatusPresets): One-click status presets.
use serein_extension_sdk::{export, Element, Invocation, Output};

fn handle(inv: Invocation) -> Output {
    if inv.action == "presets-panel" {
        Output {
            panel: vec![
                Element::Text { text: "Status Presets".into() },
                Element::Text { text: "Quick: 💻 Deep Work | 🎮 Gaming | ☕ Taking a break | 🎧 In Focus".into() },
            ],
            ..Default::default()
        }
    } else {
        Output::default()
    }
}

export!(handle);
