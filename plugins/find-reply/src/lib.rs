/// VenSerein (FindReply): Navigate and quote original reply target.
use serein_extension_sdk::{export, Invocation, Output};

fn handle(inv: Invocation) -> Output {
    if inv.action != "jump-reply" {
        return Output::default();
    }
    let text = inv.selected_message.as_deref().unwrap_or("");
    Output {
        replacement: Some(format!("Ref: {}", text.lines().next().unwrap_or(""))),
        ..Default::default()
    }
}

export!(handle);
