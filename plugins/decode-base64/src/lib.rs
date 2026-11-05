/// VenSerein (DecodeBase64): Decode base64 strings from chat context.
use serein_extension_sdk::{export, Invocation, Output};

fn handle(inv: Invocation) -> Output {
    if inv.action != "decode" {
        return Output::default();
    }
    let text = match inv.composer.as_deref().or(inv.selected_message.as_deref()) {
        Some(t) => t.trim().to_owned(),
        None => return Output { replacement: Some("No text selected.".into()), ..Default::default() },
    };
    let clean: String = text.split_whitespace().collect();
    let result = match base64_decode(&clean) {
        Ok(bytes) => match String::from_utf8(bytes.clone()) {
            Ok(s) => format!("Decoded: {}", s),
            Err(_) => format!(
                "Decoded (hex): {}",
                bytes.iter().map(|b| format!("{:02x}", b)).collect::<Vec<_>>().join(" ")
            ),
        },
        Err(_) => "Not valid base64.".into(),
    };
    Output { replacement: Some(result), ..Default::default() }
}

/// Minimal standard-alphabet base64 decoder.
fn base64_decode(input: &str) -> Result<Vec<u8>, ()> {
    const ALPHA: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let bytes: Vec<u8> = input.bytes().filter(|&b| b != b'=').collect();
    if bytes.len() % 4 == 1 {
        return Err(());
    }
    let idx: Vec<u8> = bytes
        .iter()
        .map(|&b| ALPHA.iter().position(|&a| a == b).map(|i| i as u8).unwrap_or(255))
        .collect();
    if idx.iter().any(|&v| v == 255) {
        return Err(());
    }
    let mut out = Vec::with_capacity(bytes.len() * 3 / 4);
    for chunk in idx.chunks(4) {
        let b0 = chunk[0] as u32;
        let b1 = chunk[1] as u32;
        out.push(((b0 << 2) | (b1 >> 4)) as u8);
        if chunk.len() >= 3 {
            let b2 = chunk[2] as u32;
            out.push(((b1 << 4) | (b2 >> 2)) as u8);
        }
        if chunk.len() >= 4 {
            let b2 = chunk[2] as u32;
            let b3 = chunk[3] as u32;
            out.push(((b2 << 6) | b3) as u8);
        }
    }
    Ok(out)
}

export!(handle);
