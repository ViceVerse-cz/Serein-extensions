/// VenSerein (VoiceChannelLog): Local voice room event audit trail.
use serein_extension_sdk::{export, Element, Invocation, Output};

fn handle(inv: Invocation) -> Output {
    if inv.action == "voice-log-panel" {
        Output {
            panel: vec![
                Element::Text { text: "Voice Channel Session Log".into() },
                Element::Text { text: "Session events recorded locally with timestamps.".into() },
            ],
            ..Default::default()
        }
    } else {
        Output::default()
    }
}

export!(handle);
