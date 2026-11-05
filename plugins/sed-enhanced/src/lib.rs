/// VenSerein (SedEnhanced): Apply regex sed-style replacements to messages.
use serein_extension_sdk::{export, Invocation, Output, Element};

fn handle(inv: Invocation) -> Output {
    match inv.action.as_str() {
        "sed-replace" => {
            let pattern = inv.composer.as_deref().unwrap_or("");
            let target = inv.selected_message.as_deref().unwrap_or(pattern);
            if let Some(result) = apply_sed(pattern, target) {
                Output { replacement: Some(result), ..Default::default() }
            } else {
                Output {
                    replacement: Some("Invalid sed pattern. Use: s/from/to/".into()),
                    ..Default::default()
                }
            }
        }
        "history" => Output {
            panel: vec![Element::Text { text: "Sed replacement history (stored locally).".into() }],
            ..Default::default()
        },
        _ => Output::default(),
    }
}

fn apply_sed(pattern: &str, target: &str) -> Option<String> {
    let p = pattern.trim();
    if !p.starts_with("s/") {
        return None;
    }
    let rest = &p[2..];
    let parts: Vec<&str> = rest.splitn(3, '/').collect();
    if parts.len() < 2 {
        return None;
    }
    let from = parts[0];
    let to = parts[1];
    let flags = parts.get(2).copied().unwrap_or("");
    let result = if flags.contains('g') {
        target.replace(from, to)
    } else if let Some(pos) = target.find(from) {
        format!("{}{}{}", &target[..pos], to, &target[pos + from.len()..])
    } else {
        target.to_owned()
    };
    Some(result)
}

export!(handle);
