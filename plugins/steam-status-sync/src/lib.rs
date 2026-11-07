/// VenSerein (SteamStatusSync): Bridge Steam games to Discord RPC.
use serein_extension_sdk::{export, Element, Invocation, Output};

fn handle(inv: Invocation) -> Output {
    if inv.action == "steam-sync-panel" {
        Output {
            panel: vec![
                Element::Text { text: "Steam Activity Bridge".into() },
                Element::TextInput { id: "steam_id".into(), label: "SteamID64 / Profile URL".into(), value: String::new() },
            ],
            ..Default::default()
        }
    } else {
        Output::default()
    }
}

export!(handle);
