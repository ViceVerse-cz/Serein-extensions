/// VenSerein (PolishWording): Auto-fix common typos and punctuation before sending.
use serein_extension_sdk::{export, Invocation, Output};

fn handle(inv: Invocation) -> Output {
    match inv.action.as_str() {
        "polish" => {
            let text = match inv.composer.as_deref() {
                Some(t) => t,
                None => return Output::default(),
            };
            Output { replacement: Some(polish(text)), ..Default::default() }
        }
        "settings" => Output {
            panel: vec![
                serein_extension_sdk::Element::Text { text: "Polish Wording Settings".into() },
                serein_extension_sdk::Element::Text { text: "Auto-fix common typos and punctuation on send.".into() },
            ],
            ..Default::default()
        },
        _ => Output::default(),
    }
}

fn polish(text: &str) -> String {
    let fixes: &[(&str, &str)] = &[
        ("teh ", "the "),
        ("recieve", "receive"),
        ("occured", "occurred"),
        ("seperate", "separate"),
        ("definately", "definitely"),
        ("alot", "a lot"),
        ("untill", "until"),
        ("wierd", "weird"),
        ("beleive", "believe"),
        ("freind", "friend"),
        ("  ", " "),
    ];
    let mut out = text.to_owned();
    for (from, to) in fixes {
        out = out.replace(from, to);
    }
    // Capitalize first letter
    let mut chars = out.chars();
    match chars.next() {
        None => out,
        Some(c) => c.to_uppercase().to_string() + chars.as_str(),
    }
}

export!(handle);
