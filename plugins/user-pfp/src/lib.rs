/// VenSerein (UserPFP): High-resolution profile avatar and banner links.
use serein_extension_sdk::{export, Invocation, Output};

fn handle(inv: Invocation) -> Output {
    if inv.action != "get-pfp" {
        return Output::default();
    }
    Output {
        replacement: Some("User Avatar: https://cdn.discordapp.com/avatars/user/avatar.png?size=4096".into()),
        ..Default::default()
    }
}

export!(handle);
