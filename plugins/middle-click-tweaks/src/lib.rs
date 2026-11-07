/// VenSerein (MiddleClickTweaks): Custom middle-click gesture actions.
use serein_extension_sdk::{export, Element, Invocation, Output};

fn handle(inv: Invocation) -> Output {
    if inv.action == "middle-click-config" {
        Output {
            panel: vec![
                Element::Text { text: "Middle Click Action Mapping".into() },
                Element::Text { text: "Middle click message: Open raw view / Quote in composer".into() },
            ],
            ..Default::default()
        }
    } else {
        Output::default()
    }
}

export!(handle);
