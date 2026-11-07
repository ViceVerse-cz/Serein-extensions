/// VenSerein (StickerBlocker): Collapse or block large sticker images in chat.
use serein_extension_sdk::{export, Element, Invocation, Output};

fn handle(inv: Invocation) -> Output {
    if inv.action == "sticker-block-settings" {
        Output {
            panel: vec![
                Element::Text { text: "Sticker Display Mode".into() },
                Element::Text { text: "Mode: Compact label (replaces large animation with [Sticker: name]).".into() },
            ],
            ..Default::default()
        }
    } else {
        Output::default()
    }
}

export!(handle);
