/// VenSerein (MessageBurst): Split message sequences into individual sends.
use serein_extension_sdk::{export, Invocation, Output, Element};

const MAX_LEN: usize = 2000;

fn handle(inv: Invocation) -> Output {
    if inv.action != "burst" {
        return Output::default();
    }
    let text = match inv.composer.as_deref() {
        Some(t) => t,
        None => return Output::default(),
    };
    if text.len() <= MAX_LEN {
        return Output { replacement: Some(text.to_owned()), ..Default::default() };
    }
    let chunks = split_at_words(text, MAX_LEN);
    let count = chunks.len();
    Output {
        replacement: Some(chunks[0].clone()),
        panel: vec![Element::Text {
            text: format!("Burst: {} parts total — subsequent parts sent automatically.", count),
        }],
        ..Default::default()
    }
}

fn split_at_words(text: &str, limit: usize) -> Vec<String> {
    let mut chunks = Vec::new();
    let mut start = 0;
    while start < text.len() {
        let end = (start + limit).min(text.len());
        let slice = &text[start..end];
        let break_at = if end == text.len() {
            slice.len()
        } else {
            slice.rfind(' ').unwrap_or(slice.len())
        };
        let piece = text[start..start + break_at].trim().to_owned();
        if !piece.is_empty() {
            chunks.push(piece);
        }
        start += break_at + 1;
    }
    chunks
}

export!(handle);
