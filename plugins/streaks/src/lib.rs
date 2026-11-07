/// VenSerein (Streaks): Daily communication streak tracker.
use serein_extension_sdk::{export, Invocation, Output};

fn handle(inv: Invocation) -> Output {
    if inv.action != "streak-status" {
        return Output::default();
    }
    let text = inv.composer.as_deref().unwrap_or("");
    Output {
        replacement: Some(format!("{} 🔥 1-day streak", text).trim().to_string()),
        ..Default::default()
    }
}

export!(handle);
