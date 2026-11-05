/// VenSerein (SongSpotlight): Showcase a favorite song on profile.
use serein_extension_sdk::{export, Element, Invocation, Output};

fn handle(inv: Invocation) -> Output {
    if inv.action == "spotlight-settings" {
        Output {
            panel: vec![
                Element::Text { text: "Profile Song Spotlight".into() },
                Element::TextInput { id: "track_url".into(), label: "Featured Track URL".into(), value: String::new() },
            ],
            ..Default::default()
        }
    } else {
        Output::default()
    }
}

export!(handle);
