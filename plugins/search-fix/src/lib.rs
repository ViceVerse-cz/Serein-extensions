/// VenSerein (SearchFix): Advanced search query syntax builder.
use serein_extension_sdk::{export, Element, Invocation, Output};

fn handle(inv: Invocation) -> Output {
    match inv.action.as_str() {
        "insert-filter" => Output {
            replacement: Some("from:me has:link ".into()),
            ..Default::default()
        },
        "search-help" => Output {
            panel: vec![
                Element::Text { text: "Discord Search Syntax".into() },
                Element::Text { text: "from:user | mentions:user | has:link/image/file | before:date".into() },
            ],
            ..Default::default()
        },
        _ => Output::default(),
    }
}

export!(handle);
