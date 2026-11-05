/// VenSerein (FastDeleteChannels): Fast channel deletion without typing channel name.
use serein_extension_sdk::{export, Element, Invocation, Output};

fn handle(inv: Invocation) -> Output {
    if inv.action == "fast-delete-toggle" {
        Output {
            panel: vec![
                Element::Text { text: "Fast Delete Channels: Active".into() },
                Element::Text { text: "Skips manual channel name confirmation requirement.".into() },
            ],
            ..Default::default()
        }
    } else {
        Output::default()
    }
}

export!(handle);
