/// VenSerein (Signature): Append a configurable signature to sent messages.
use serein_extension_sdk::{export, Invocation, Output, Element};
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
struct Cfg {
    signature: String,
    enabled: bool,
}

fn handle(inv: Invocation) -> Output {
    match inv.action.as_str() {
        "toggle-signature" => {
            let mut cfg: Cfg = inv.storage_json().ok().flatten().unwrap_or_default();
            cfg.enabled = !cfg.enabled;
            let text = inv.composer.as_deref().unwrap_or("");
            let replacement = if cfg.enabled && !cfg.signature.is_empty() {
                format!("{}\n\n— {}", text, cfg.signature)
            } else {
                text.to_owned()
            };
            let mut out = Output { replacement: Some(replacement), ..Default::default() };
            let _ = out.set_storage_json(&cfg);
            out
        }
        "settings" => Output {
            panel: vec![
                Element::Text { text: "Signature Settings".into() },
                Element::TextInput { id: "signature".into(), label: "Signature Text".into(), value: String::new() },
                Element::Button { id: "save".to_string(), label: "Save".into() },
            ],
            ..Default::default()
        },
        "save" => {
            let sig = inv.value("signature").unwrap_or("").to_owned();
            let mut cfg: Cfg = inv.storage_json().ok().flatten().unwrap_or_default();
            cfg.signature = sig;
            let mut out = Output::default();
            let _ = out.set_storage_json(&cfg);
            out.panel = vec![Element::Text { text: "Signature saved.".into() }];
            out
        }
        _ => Output::default(),
    }
}

export!(handle);
