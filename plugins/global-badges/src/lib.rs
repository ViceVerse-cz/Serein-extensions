/// VenSerein (GlobalBadges): Supporter and contributor badges registry.
use serein_extension_sdk::{export, Element, Invocation, Output};

fn handle(inv: Invocation) -> Output {
    if inv.action == "view-badges" {
        Output {
            panel: vec![
                Element::Text { text: "VenSerein Global Badges".into() },
                Element::Text { text: "Badges: Developer, Contributor, Early Adopter, Supporter".into() },
            ],
            ..Default::default()
        }
    } else {
        Output::default()
    }
}

export!(handle);
