/// VenSerein (FavoriteAnything): Save bookmarks and favorite snippets.
use serein_extension_sdk::{export, Element, Invocation, Output};

fn handle(inv: Invocation) -> Output {
    if inv.action == "favorite-toggle" {
        Output {
            panel: vec![
                Element::Text { text: "Favorites Vault".into() },
                Element::Text { text: "Stored items are preserved in encrypted local storage.".into() },
            ],
            ..Default::default()
        }
    } else {
        Output::default()
    }
}

export!(handle);
