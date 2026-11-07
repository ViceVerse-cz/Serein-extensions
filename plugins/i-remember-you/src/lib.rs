/// VenSerein (IRememberYou): Remember historical user identities and nicknames.
use serein_extension_sdk::{export, Element, Invocation, Output};

fn handle(inv: Invocation) -> Output {
    if inv.action == "user-cache" {
        Output {
            panel: vec![
                Element::Text { text: "User Identity Archive".into() },
                Element::Text { text: "Cached historical profiles: active".into() },
            ],
            ..Default::default()
        }
    } else {
        Output::default()
    }
}

export!(handle);
