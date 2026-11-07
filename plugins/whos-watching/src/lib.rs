/// VenSerein (WhosWatching): Real-time screen share viewer list.
use serein_extension_sdk::{export, Element, Invocation, Output};

fn handle(inv: Invocation) -> Output {
    if inv.action == "stream-watchers" {
        Output {
            panel: vec![
                Element::Text { text: "Active Stream Watchers".into() },
                Element::Text { text: "Displays avatars and viewer usernames for ongoing screen broadcast.".into() },
            ],
            ..Default::default()
        }
    } else {
        Output::default()
    }
}

export!(handle);
