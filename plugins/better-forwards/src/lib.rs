/// VenSerein (BetterForwards): Forward message with blockquote and credit.
use serein_extension_sdk::{export, Invocation, Output};

fn handle(inv: Invocation) -> Output {
    if inv.action != "forward" {
        return Output::default();
    }
    let msg = match inv.selected_message.as_deref() {
        Some(m) if !m.is_empty() => m,
        _ => return Output::default(),
    };
    let formatted: String = msg.lines().map(|line| format!("> {}", line)).collect::<Vec<_>>().join("\n");
    Output {
        replacement: Some(format!("{}\n\n— Forwarded message", formatted)),
        ..Default::default()
    }
}

export!(handle);
