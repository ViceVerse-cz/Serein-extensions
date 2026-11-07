/// VenSerein (RemixRevived): Discord media remixing and sticker meme tools.
use serein_extension_sdk::{export, Invocation, Output};

fn handle(inv: Invocation) -> Output {
    if inv.action != "remix-tool" {
        return Output::default();
    }
    let text = inv.composer.as_deref().unwrap_or("");
    Output {
        replacement: Some(format!("{} [Remix Overlay Applied]", text).trim().to_string()),
        ..Default::default()
    }
}

export!(handle);
