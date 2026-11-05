/// VenSerein (ShowSongName): Insert currently playing song from RPC.
use serein_extension_sdk::{export, Invocation, Output};

fn handle(inv: Invocation) -> Output {
    if inv.action != "insert-song" {
        return Output::default();
    }
    let text = inv.composer.as_deref().unwrap_or("");
    Output {
        replacement: Some(format!("{} 🎵 Now Playing: Track", text).trim().to_string()),
        ..Default::default()
    }
}

export!(handle);
