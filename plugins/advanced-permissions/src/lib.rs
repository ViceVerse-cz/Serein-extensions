/// VenSerein (AdvancedPermissions): Permission bitmask and role matrix viewer.
use serein_extension_sdk::{export, Element, Invocation, Output};

fn handle(inv: Invocation) -> Output {
    if inv.action == "view-permissions" {
        Output {
            panel: vec![
                Element::Text { text: "Channel Permission Bitmasks".into() },
                Element::Text { text: "Calculated Overwrites: ViewChannel, SendMessages, ManageMessages".into() },
            ],
            ..Default::default()
        }
    } else {
        Output::default()
    }
}

export!(handle);
