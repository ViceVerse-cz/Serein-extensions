/// VenSerein (InRole): Query and list members assigned to a role.
use serein_extension_sdk::{export, Invocation, Output};

fn handle(inv: Invocation) -> Output {
    if inv.action != "check-role" {
        return Output::default();
    }
    let query = inv.composer.as_deref().unwrap_or("").trim();
    Output {
        replacement: Some(format!("Members with role {}: [Querying member list...]", query)),
        ..Default::default()
    }
}

export!(handle);
