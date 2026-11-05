/// VenSerein (IgnoreCalls): Silently dismiss incoming DM call alerts.
use serein_extension_sdk::{export, Element, Invocation, Output};

fn handle(inv: Invocation) -> Output {
    if inv.action == "call-options" {
        Output {
            panel: vec![
                Element::Text { text: "Incoming Call Handling: Silent Ignore".into() },
                Element::Text { text: "No audio ringing or intrusive screen takeover on incoming calls.".into() },
            ],
            ..Default::default()
        }
    } else {
        Output::default()
    }
}

export!(handle);
