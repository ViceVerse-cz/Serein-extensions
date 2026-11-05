/// VenSerein (TidalEmbeds): Rich player format for Tidal URLs.
use serein_extension_sdk::{export, Invocation, Output};

fn handle(inv: Invocation) -> Output {
    if inv.action != "tidal-card" {
        return Output::default();
    }
    let text = inv.composer.as_deref().unwrap_or("").trim();
    if text.contains("tidal.com") {
        Output {
            replacement: Some(format!("▶ Tidal Music: {}", text)),
            ..Default::default()
        }
    } else {
        Output::default()
    }
}

export!(handle);
