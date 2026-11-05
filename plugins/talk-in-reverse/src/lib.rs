/// VenSerein (TalkInReverse): Reverse text before sending.
use serein_extension_sdk::{export, Invocation, Output};

fn handle(inv: Invocation) -> Output {
    if inv.action != "reverse" {
        return Output::default();
    }
    let text = inv.composer.as_deref().unwrap_or("");
    let reversed: String = text.chars().rev().collect();
    Output { replacement: Some(reversed), ..Default::default() }
}

export!(handle);
