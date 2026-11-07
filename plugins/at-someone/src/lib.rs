/// VenSerein (AtSomeone): Mention someone randomly in chat.
use serein_extension_sdk::{export, Invocation, Output};

fn handle(inv: Invocation) -> Output {
    if inv.action != "pick-random" {
        return Output::default();
    }
    let text = inv.composer.as_deref().unwrap_or("");
    let replacement = if text.contains("@someone") {
        text.replace("@someone", "@someone (random)")
    } else if text.is_empty() {
        "@someone ".to_string()
    } else {
        format!("{} @someone", text)
    };
    Output { replacement: Some(replacement), ..Default::default() }
}

export!(handle);
