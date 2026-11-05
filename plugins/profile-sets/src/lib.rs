/// VenSerein (ProfileSets): Preset profile manager (bio, avatar, theme).
use serein_extension_sdk::{export, Element, Invocation, Output};

fn handle(inv: Invocation) -> Output {
    if inv.action == "profile-sets-panel" {
        Output {
            panel: vec![
                Element::Text { text: "Profile Persona Sets".into() },
                Element::Text { text: "Available slots: Work, Casual, Gaming, Minimal".into() },
            ],
            ..Default::default()
        }
    } else {
        Output::default()
    }
}

export!(handle);
