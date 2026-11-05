/// VenSerein (MoreUserTags): Custom user badges and category tags.
use serein_extension_sdk::{export, Element, Invocation, Output};

fn handle(inv: Invocation) -> Output {
    if inv.action == "user-tags-panel" {
        Output {
            panel: vec![
                Element::Text { text: "User Tag Registry".into() },
                Element::Text { text: "Assign custom tags and colors to contacts in chat view.".into() },
            ],
            ..Default::default()
        }
    } else {
        Output::default()
    }
}

export!(handle);
