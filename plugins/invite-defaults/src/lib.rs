/// VenSerein (InviteDefaults): Default duration and max uses for new invites.
use serein_extension_sdk::{export, Element, Invocation, Output};

fn handle(inv: Invocation) -> Output {
    if inv.action == "invite-config" {
        Output {
            panel: vec![
                Element::Text { text: "Default Invite Configuration".into() },
                Element::Text { text: "Expiration: 7 days | Max uses: Unlimited | Temporary membership: Off".into() },
            ],
            ..Default::default()
        }
    } else {
        Output::default()
    }
}

export!(handle);
