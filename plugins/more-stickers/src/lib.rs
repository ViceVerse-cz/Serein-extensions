/// VenSerein (MoreStickers): Custom sticker pack manager and quick picker.
use serein_extension_sdk::{export, Element, Invocation, Output};

fn handle(inv: Invocation) -> Output {
    if inv.action == "sticker-panel" {
        Output {
            panel: vec![
                Element::Text { text: "Custom Sticker Collections".into() },
                Element::Text { text: "Send non-Nitro stickers as high-res images automatically.".into() },
            ],
            ..Default::default()
        }
    } else {
        Output::default()
    }
}

export!(handle);
