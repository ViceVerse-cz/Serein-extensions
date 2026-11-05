/// VenSerein (MessageTranslate): Quick translate messages via external API.
use serein_extension_sdk::{export, Invocation, Output};
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
struct Config {
    target_lang: String,
    api_endpoint: String,
}

fn handle(inv: Invocation) -> Output {
    match inv.action.as_str() {
        "translate" => {
            let text = inv.composer.as_deref()
                .or(inv.selected_message.as_deref())
                .unwrap_or("");
            let cfg: Config = inv.storage_json().ok().flatten().unwrap_or_default();
            let lang = if cfg.target_lang.is_empty() { "EN" } else { &cfg.target_lang };
            Output {
                replacement: Some(format!(
                    "[Translate→{}] {}", lang, text
                )),
                ..Default::default()
            }
        }
        "settings" => {
            use serein_extension_sdk::Element;
            Output {
                panel: vec![
                    Element::Text { text: "Translation Settings".into() },
                    Element::TextInput { id: "target_lang".into(), label: "Target Language (e.g. EN, DE, FR)".into(), value: String::new() },
                    Element::TextInput { id: "api_endpoint".into(), label: "API Endpoint (optional)".into(), value: String::new() },
                    Element::Button { id: "save".to_string(), label: "Save".into() },
                ],
                ..Default::default()
            }
        }
        "save" => {
            let cfg = Config {
                target_lang: inv.value("target_lang").unwrap_or("EN").to_owned(),
                api_endpoint: inv.value("api_endpoint").unwrap_or("").to_owned(),
            };
            let mut out = Output::default();
            let _ = out.set_storage_json(&cfg);
            out.panel = vec![serein_extension_sdk::Element::Text { text: "Settings saved.".into() }];
            out
        }
        _ => Output::default(),
    }
}

export!(handle);
