/// VenSerein (MusicControls): Rich Presence and media player control panel.
use serein_extension_sdk::{export, Element, Invocation, Output};

fn handle(inv: Invocation) -> Output {
    if inv.action == "media-controls" {
        Output {
            panel: vec![
                Element::Text { text: "Active Media Player Controls".into() },
                Element::Text { text: "Controls: ⏵ Play / ⏸ Pause / ⏭ Skip / ⏮ Previous".into() },
            ],
            ..Default::default()
        }
    } else {
        Output::default()
    }
}

export!(handle);
