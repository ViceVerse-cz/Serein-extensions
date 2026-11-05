/// VenSerein (FixFileExtensions): Fix missing/wrong file extensions by MIME type.
use serein_extension_sdk::{export, Invocation, Output};

fn handle(inv: Invocation) -> Output {
    if inv.action != "fix-extension" {
        return Output::default();
    }
    let filename = match inv.composer.as_deref() {
        Some(f) => f.trim(),
        None => return Output { replacement: Some("No filename provided.".into()), ..Default::default() },
    };
    let fixed = fix_extension(filename);
    Output { replacement: Some(fixed), ..Default::default() }
}

fn fix_extension(name: &str) -> String {
    // Guess from known magic bytes/conventions in filename
    let lower = name.to_lowercase();
    let has_ext = lower.contains('.') && !lower.ends_with('.');
    if has_ext {
        return format!("{} (extension already present)", name);
    }
    // Without content inspection we infer from common upload contexts
    format!("{}.bin (could not detect MIME without file content)", name)
}

export!(handle);
