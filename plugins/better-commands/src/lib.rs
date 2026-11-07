/// VenSerein (BetterCommands): Auto-fill command arguments and shortcuts.
use serein_extension_sdk::{export, Invocation, Output};

fn handle(inv: Invocation) -> Output {
    if inv.action != "fill-args" {
        return Output::default();
    }
    let text = inv.composer.as_deref().unwrap_or("");
    if text.starts_with('/') {
        Output {
            replacement: Some(format!("{} [options]", text.trim())),
            ..Default::default()
        }
    } else {
        Output::default()
    }
}

export!(handle);
