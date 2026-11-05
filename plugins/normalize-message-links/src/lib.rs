/// VenSerein (NormalizeMessageLinks): Rewrite ptb/canary links to canonical discord.com.
use serein_extension_sdk::{export, Invocation, Output};

fn handle(inv: Invocation) -> Output {
    if inv.action != "normalize" {
        return Output::default();
    }
    let text = match inv.composer.as_deref() {
        Some(t) => t,
        None => return Output::default(),
    };
    let normalized = normalize(text);
    Output { replacement: Some(normalized), ..Default::default() }
}

fn normalize(text: &str) -> String {
    text.replace("ptb.discord.com", "discord.com")
        .replace("canary.discord.com", "discord.com")
        .replace("discord.gg/", "discord.com/invite/")
}

export!(handle);
