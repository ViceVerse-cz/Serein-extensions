/// VenSerein (AutoZipper): Auto-zip batches of files before upload.
use serein_extension_sdk::{export, Invocation, Output};

fn handle(inv: Invocation) -> Output {
    match inv.action.as_str() {
        "toggle-auto-zip" => {
            let enabled = inv.storage.as_deref() == Some("1");
            Output {
                storage: Some(if enabled { "0" } else { "1" }.into()),
                replacement: Some(if enabled {
                    "Auto-Zip disabled.".into()
                } else {
                    "Auto-Zip enabled — files will be zipped before upload.".into()
                }),
                ..Default::default()
            }
        }
        _ => Output::default(),
    }
}

export!(handle);
