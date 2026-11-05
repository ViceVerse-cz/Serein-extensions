/// VenSerein (MessageNotifier): Per-user notification triggers and sounds.
use serein_extension_sdk::{export, Element, Invocation, Output};

fn handle(inv: Invocation) -> Output {
    if inv.action == "notifier-rules" {
        Output {
            panel: vec![
                Element::Text { text: "Granular Notification Rules".into() },
                Element::Text { text: "Configure distinctive chime sounds per friend or server channel.".into() },
            ],
            ..Default::default()
        }
    } else {
        Output::default()
    }
}

export!(handle);
