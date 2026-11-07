/// VenSerein (WriteUpperCase): Format text into Title Case, UPPER CASE, or sPoNgE cAsE.
use serein_extension_sdk::{export, Invocation, Output};

fn handle(inv: Invocation) -> Output {
    let text = inv.composer.as_deref().unwrap_or("");
    let result = match inv.action.as_str() {
        "title-case" => to_title(text),
        "upper-case" => text.to_uppercase(),
        "sponge-case" => to_sponge(text),
        _ => return Output::default(),
    };
    Output { replacement: Some(result), ..Default::default() }
}

fn to_title(text: &str) -> String {
    text.split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(f) => {
                    let mut s = String::new();
                    for up in f.to_uppercase() {
                        s.push(up);
                    }
                    s.push_str(chars.as_str());
                    s
                }
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn to_sponge(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for (i, c) in text.chars().enumerate() {
        if i % 2 == 0 {
            for up in c.to_uppercase() {
                out.push(up);
            }
        } else {
            for low in c.to_lowercase() {
                out.push(low);
            }
        }
    }
    out
}

export!(handle);
