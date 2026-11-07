/// VenSerein (BetterImageEditor): Image transform and crop settings.
use serein_extension_sdk::{export, Element, Invocation, Output};

fn handle(inv: Invocation) -> Output {
    if inv.action == "editor" {
        Output {
            panel: vec![
                Element::Text { text: "Better Image Editor".into() },
                Element::Text { text: "Quick transforms: Rotate 90°, Flip Horizontal, Compress Before Send".into() },
            ],
            ..Default::default()
        }
    } else {
        Output::default()
    }
}

export!(handle);
