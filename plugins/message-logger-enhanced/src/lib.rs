/// VenSerein (MessageLoggerEnhanced): Deleted and edited message audit logger.
use serein_extension_sdk::{export, Element, Invocation, Output};

fn handle(inv: Invocation) -> Output {
    if inv.action == "logger-panel" {
        Output {
            preserve_deleted_messages: true,
            panel: vec![
                Element::Text { text: "Message Logger Enhanced".into() },
                Element::Text { text: "Deleted messages preservation: ACTIVE in memory.".into() },
            ],
            ..Default::default()
        }
    } else {
        Output {
            preserve_deleted_messages: true,
            ..Default::default()
        }
    }
}

export!(handle);
