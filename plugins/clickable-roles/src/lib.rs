/// VenSerein (ClickableRoles): Clickable role mentions in chat.
use serein_extension_sdk::{export, Element, Invocation, Output};

fn handle(inv: Invocation) -> Output {
    if inv.action == "role-settings" {
        Output {
            panel: vec![
                Element::Text { text: "Clickable Roles: Enabled".into() },
                Element::Text { text: "Click a role to copy ID, shift-click to view members.".into() },
            ],
            ..Default::default()
        }
    } else {
        Output::default()
    }
}

export!(handle);
