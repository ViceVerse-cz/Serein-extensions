/// VenSerein (GitHubRepos): Expand GitHub repo URLs into summary cards.
use serein_extension_sdk::{export, Invocation, Output};

fn handle(inv: Invocation) -> Output {
    if inv.action != "expand-repo" {
        return Output::default();
    }
    let text = inv.composer.as_deref().unwrap_or("");
    let rep = text.split_whitespace().find(|w| w.contains("github.com/")).unwrap_or(text);
    Output {
        replacement: Some(format!("{} (GitHub Repository)", rep)),
        ..Default::default()
    }
}

export!(handle);
