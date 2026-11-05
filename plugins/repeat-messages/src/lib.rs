/// VenSerein (RepeatMessages): Duplicate or repeat messages in chat.
use serein_extension_sdk::{export, Invocation, Output};

fn handle(inv: Invocation) -> Output {
    if inv.action != "repeat" {
        return Output::default();
    }
    let text = inv.selected_message.as_deref().or(inv.composer.as_deref()).unwrap_or("");
    Output { replacement: Some(text.to_string()), ..Default::default() }
}

export!(handle);
