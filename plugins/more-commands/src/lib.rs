/// VenSerein (MoreCommands): Handy text shortcuts like /shrug and /tableflip.
use serein_extension_sdk::{export, Invocation, Output};

fn handle(inv: Invocation) -> Output {
    let text = inv.composer.as_deref().unwrap_or("");
    let replacement = match inv.action.as_str() {
        "shrug" => format!("{} ¯\\_(ツ)_/¯", text).trim().to_string(),
        "tableflip" => format!("{} (╯°□°)╯︵ ┻━┻", text).trim().to_string(),
        _ => return Output::default(),
    };
    Output { replacement: Some(replacement), ..Default::default() }
}

export!(handle);
