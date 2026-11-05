/// VenSerein (GifMaker): Convert clips to animated GIF formats.
use serein_extension_sdk::{export, Element, Invocation, Output};

fn handle(inv: Invocation) -> Output {
    if inv.action == "maker-panel" {
        Output {
            panel: vec![
                Element::Text { text: "GIF Maker Utility".into() },
                Element::Text { text: "Settings: 15 FPS, max 50 frames, optimized palette".into() },
            ],
            ..Default::default()
        }
    } else {
        Output::default()
    }
}

export!(handle);
