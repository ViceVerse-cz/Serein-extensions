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
            let mut c = word.chars();
            match c.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().to_string() + c.as_str(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn to_sponge(text: &str) -> String {
    text.chars()
        .enumerate()
        .map(|(i, c)| if i % 2 == 0 { c.to_uppercase().next().unwrap_or(c) } else { c.to_lowercase().next().unwrap_or(c) })
        .collect()
}

export!(handle);
