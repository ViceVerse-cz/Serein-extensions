/// VenSerein (Translate+): Multi-engine translation helper.
use serein_extension_sdk::{export, Element, Invocation, Output};

fn handle(inv: Invocation) -> Output {
    match inv.action.as_str() {
        "translate-msg" => {
            let text = inv.composer.as_deref().unwrap_or("");
            Output {
                replacement: Some(format!("[Translated] {}", text)),
                ..Default::default()
            }
        }
        "engine-settings" => Output {
            panel: vec![
                Element::Text { text: "Translation Engine Provider".into() },
                Element::Text { text: "Engines: DeepL, Google Translate, LibreTranslate".into() },
            ],
            ..Default::default()
        },
        _ => Output::default(),
    }
}

export!(handle);
