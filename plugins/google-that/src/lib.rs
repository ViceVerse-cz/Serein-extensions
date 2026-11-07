/// VenSerein (GoogleThat): Generate Google search query links.
use serein_extension_sdk::{export, Invocation, Output};

fn handle(inv: Invocation) -> Output {
    if inv.action != "google-search" {
        return Output::default();
    }
    let query = inv.composer.as_deref().unwrap_or("").trim();
    if query.is_empty() {
        return Output::default();
    }
    let encoded = query.replace(' ', "+");
    let link = format!("https://www.google.com/search?q={}", encoded);
    Output { replacement: Some(link), ..Default::default() }
}

export!(handle);
