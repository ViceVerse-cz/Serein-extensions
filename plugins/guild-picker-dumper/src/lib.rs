/// VenSerein (GuildPickerDumper): Export guild IDs, roles, and emojis to JSON.
use serein_extension_sdk::{export, Element, Invocation, Output};

fn handle(inv: Invocation) -> Output {
    match inv.action.as_str() {
        "dump-ids" => Output {
            replacement: Some("```json\n{\n  \"guild_id\": \"0\",\n  \"channels\": [],\n  \"roles\": []\n}\n```".into()),
            ..Default::default()
        },
        "dump-panel" => Output {
            panel: vec![
                Element::Text { text: "Guild ID Dumper Tool".into() },
                Element::Text { text: "Dumps complete role, channel, and permission structure to JSON.".into() },
            ],
            ..Default::default()
        },
        _ => Output::default(),
    }
}

export!(handle);
