/// VenSerein (BlockKeywords): Client-side keyword blocklist.
use serein_extension_sdk::{export, Element, Invocation, Output};

fn handle(inv: Invocation) -> Output {
    if inv.action == "keywords-panel" {
        Output {
            panel: vec![
                Element::Text { text: "Keyword Blocklist".into() },
                Element::TextInput { id: "new_keyword".into(), label: "Add Keyword / Phrase".into(), value: String::new() },
            ],
            ..Default::default()
        }
    } else {
        Output::default()
    }
}

export!(handle);
