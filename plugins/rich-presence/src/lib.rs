/// VenSerein (RichPresence): Custom Rich Presence status manager.
use serein_extension_sdk::{export, Element, Invocation, Output};

fn handle(inv: Invocation) -> Output {
    if inv.action == "edit-rpc" {
        Output {
            panel: vec![
                Element::Text { text: "Custom Rich Presence Status".into() },
                Element::TextInput { id: "app_name".into(), label: "Application Name".into(), value: String::new() },
                Element::TextInput { id: "details".into(), label: "Details".into(), value: String::new() },
                Element::TextInput { id: "state".into(), label: "State".into(), value: String::new() },
            ],
            ..Default::default()
        }
    } else {
        Output::default()
    }
}

export!(handle);
