/// VenSerein (PingNotifications): Priority mention highlights and chime sounds.
use serein_extension_sdk::{export, Element, Invocation, Output};

fn handle(inv: Invocation) -> Output {
    if inv.action == "ping-settings" {
        Output {
            panel: vec![
                Element::Text { text: "Mention & Ping Alert Settings".into() },
                Element::Text { text: "Visual badge pulse: ON | Custom ping audio tone: ON".into() },
            ],
            ..Default::default()
        }
    } else {
        Output::default()
    }
}

export!(handle);
