/// VenSerein (ExportMessages): Export chat selection to Markdown format.
use serein_extension_sdk::{export, Invocation, Output};

fn handle(inv: Invocation) -> Output {
    if inv.action != "export-md" {
        return Output::default();
    }
    let text = inv.selected_message.as_deref().or(inv.composer.as_deref()).unwrap_or("");
    let md = format!("### Chat Export\n\n```markdown\n{}\n```\n", text);
    Output { replacement: Some(md), ..Default::default() }
}

export!(handle);
