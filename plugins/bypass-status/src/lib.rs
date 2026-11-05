/// VenSerein (BypassStatus): Preserve custom status across presence changes.
use serein_extension_sdk::{export, Element, Invocation, Output};

fn handle(inv: Invocation) -> Output {
    if inv.action == "status-toggle" {
        Output {
            panel: vec![
                Element::Text { text: "Bypass Status: Preserving custom text across state switches.".into() },
            ],
            ..Default::default()
        }
    } else {
        Output::default()
    }
}

export!(handle);
