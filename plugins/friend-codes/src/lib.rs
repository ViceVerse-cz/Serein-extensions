/// VenSerein (FriendCodes): Share gaming platform friend codes.
use serein_extension_sdk::{export, Element, Invocation, Output};

fn handle(inv: Invocation) -> Output {
    match inv.action.as_str() {
        "paste-code" => {
            let code = inv.storage.as_deref().unwrap_or("Steam Friend Code: not configured");
            Output { replacement: Some(code.into()), ..Default::default() }
        }
        "manage-codes" => {
            Output {
                panel: vec![
                    Element::Text { text: "Gamer Friend Codes".into() },
                    Element::TextInput { id: "code_val".into(), label: "Your Codes (Steam, Switch, etc.)".into(), value: String::new() },
                ],
                ..Default::default()
            }
        }
        _ => Output::default(),
    }
}

export!(handle);
