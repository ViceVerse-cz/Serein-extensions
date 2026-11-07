/// VenSerein (SongLink): Convert track URLs to universal song.link links.
use serein_extension_sdk::{export, Invocation, Output};

fn handle(inv: Invocation) -> Output {
    if inv.action != "convert-songlink" {
        return Output::default();
    }
    let text = inv.composer.as_deref().unwrap_or("").trim();
    if text.starts_with("http") {
        Output {
            replacement: Some(format!("https://song.link/{}", text)),
            ..Default::default()
        }
    } else {
        Output::default()
    }
}

export!(handle);
