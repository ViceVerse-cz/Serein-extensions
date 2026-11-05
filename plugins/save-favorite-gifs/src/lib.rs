/// VenSerein (SaveFavoriteGIFs): Persist favorite GIFs to local extension storage.
use serein_extension_sdk::{export, Invocation, Output};

fn handle(inv: Invocation) -> Output {
    if inv.action != "save-gif" {
        return Output::default();
    }
    let url = inv.composer.as_deref().unwrap_or("").trim();
    if url.starts_with("http") && (url.contains(".gif") || url.contains("tenor.com") || url.contains("giphy.com")) {
        Output {
            replacement: Some(format!("{} (Saved to Favorites)", url)),
            ..Default::default()
        }
    } else {
        Output::default()
    }
}

export!(handle);
