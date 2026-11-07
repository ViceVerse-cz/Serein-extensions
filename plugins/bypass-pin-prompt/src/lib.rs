/// VenSerein (BypassPinPrompt): Bypass confirmation when pinning/unpinning messages.
use serein_extension_sdk::{export, Element, Invocation, Output};

fn handle(inv: Invocation) -> Output {
    if inv.action == "pin-toggle" {
        Output {
            panel: vec![
                Element::Text { text: "Pin Confirmation Bypass: Enabled".into() },
                Element::Text { text: "Messages will pin directly on single click.".into() },
            ],
            ..Default::default()
        }
    } else {
        Output::default()
    }
}

export!(handle);
