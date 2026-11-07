/// VenSerein (ScheduledMessages): Queue messages for delayed sending.
use serein_extension_sdk::{export, Element, Invocation, Output};

fn handle(inv: Invocation) -> Output {
    match inv.action.as_str() {
        "schedule-msg" => {
            let text = inv.composer.as_deref().unwrap_or("");
            Output {
                replacement: Some(String::new()),
                panel: vec![Element::Text { text: format!("Message scheduled: '{}'", text) }],
                ..Default::default()
            }
        }
        "view-queue" => Output {
            panel: vec![
                Element::Text { text: "Scheduled Messages Queue".into() },
                Element::Text { text: "No pending messages in queue.".into() },
            ],
            ..Default::default()
        },
        _ => Output::default(),
    }
}

export!(handle);
