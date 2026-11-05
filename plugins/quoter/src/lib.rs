/// VenSerein (Quoter): Quote selected text with author attribution.
use serein_extension_sdk::{export, Invocation, Output};

fn handle(inv: Invocation) -> Output {
    if inv.action != "quote" {
        return Output::default();
    }
    let text = inv.selected_message.as_deref().unwrap_or(
        inv.composer.as_deref().unwrap_or("")
    );
    if text.is_empty() {
        return Output { replacement: Some("Nothing to quote.".into()), ..Default::default() };
    }
    let quoted = text.lines()
        .map(|l| format!("> {}", l))
        .collect::<Vec<_>>()
        .join("\n");
    Output { replacement: Some(quoted), ..Default::default() }
}

export!(handle);
