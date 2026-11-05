/// VenSerein (ReplyPingControl): Default mention state on replies.
use serein_extension_sdk::{export, Element, Invocation, Output};

fn handle(inv: Invocation) -> Output {
    if inv.action == "reply-ping-settings" {
        Output {
            panel: vec![
                Element::Text { text: "Reply Mention Control".into() },
                Element::Text { text: "Default ping state: OFF (silent replies by default)".into() },
            ],
            ..Default::default()
        }
    } else {
        Output::default()
    }
}

export!(handle);
