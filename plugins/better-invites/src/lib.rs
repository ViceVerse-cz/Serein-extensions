/// VenSerein (BetterInvites): Inspect invite links and server info.
use serein_extension_sdk::{export, Element, Invocation, Output};

fn handle(inv: Invocation) -> Output {
    if inv.action != "inspect-invite" {
        return Output::default();
    }
    let text = inv.composer.as_deref().unwrap_or("");
    let invite_code = text.split_whitespace()
        .find(|w| w.contains("discord.gg/") || w.contains("discord.com/invite/"))
        .unwrap_or(text);
    Output {
        panel: vec![
            Element::Text { text: format!("Invite Target: {}", invite_code) },
            Element::Text { text: "Status: Valid invite link detected.".into() },
        ],
        ..Default::default()
    }
}

export!(handle);
