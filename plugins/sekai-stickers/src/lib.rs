/// VenSerein (SekaiStickers): Project Sekai sticker pack shortcuts.
use serein_extension_sdk::{export, Invocation, Output};

fn handle(inv: Invocation) -> Output {
    if inv.action != "sekai-picker" {
        return Output::default();
    }
    Output {
        replacement: Some(":sekai_miku_cheer:".into()),
        ..Default::default()
    }
}

export!(handle);
