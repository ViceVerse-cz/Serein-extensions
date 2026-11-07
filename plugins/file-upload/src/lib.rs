/// VenSerein (FileUpload): Inspect upload attachments and file sizes.
use serein_extension_sdk::{export, Element, Invocation, Output};

fn handle(inv: Invocation) -> Output {
    if inv.action != "check-upload" {
        return Output::default();
    }
    Output {
        panel: vec![
            Element::Text { text: "Upload Helper: standard limit 25MB (100MB with Nitro)".into() },
        ],
        ..Default::default()
    }
}

export!(handle);
