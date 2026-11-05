/// VenSerein (GifCollections): Tag and group GIF favorites into custom collections.
use serein_extension_sdk::{export, Element, Invocation, Output};

fn handle(inv: Invocation) -> Output {
    if inv.action == "open-collections" {
        Output {
            panel: vec![
                Element::Text { text: "GIF Collections".into() },
                Element::Text { text: "Organize GIFs by tags: #funny, #gaming, #reactions".into() },
            ],
            ..Default::default()
        }
    } else {
        Output::default()
    }
}

export!(handle);
