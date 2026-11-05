/// VenSerein (WigglyText): Apply Unicode wavy formatting to composed messages.
use serein_extension_sdk::{export, Invocation, Output};

fn handle(inv: Invocation) -> Output {
    if inv.action != "wiggle" {
        return Output::default();
    }
    let text = inv.composer.as_deref().unwrap_or("");
    Output { replacement: Some(wiggle(text)), ..Default::default() }
}

/// Apply a simple wavy diacritic overlay. Each char alternates between
/// combining tilde above (U+0303) and combining ring below (U+0325).
fn wiggle(text: &str) -> String {
    let mut out = String::with_capacity(text.len() * 3);
    for (i, ch) in text.chars().enumerate() {
        out.push(ch);
        if ch.is_alphabetic() {
            if i % 2 == 0 {
                out.push('\u{0303}'); // combining tilde above
            } else {
                out.push('\u{0330}'); // combining tilde below
            }
        }
    }
    out
}

export!(handle);
