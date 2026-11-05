/// VenSerein (StopAutoUnread): Retain unread markers until explicitly cleared.
use serein_extension_sdk::{export, Element, Invocation, Output};

fn handle(inv: Invocation) -> Output {
    if inv.action == "unread-policy" {
        Output {
            panel: vec![
                Element::Text { text: "Unread Marking Behavior".into() },
                Element::Text { text: "Channels only marked as read when bottom scroll position is reached.".into() },
            ],
            ..Default::default()
        }
    } else {
        Output::default()
    }
}

export!(handle);
