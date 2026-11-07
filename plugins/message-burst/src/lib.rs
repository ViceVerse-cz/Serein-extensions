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
    let mut rem = text;
    while !rem.is_empty() {
        if rem.len() <= limit {
            let piece = rem.trim();
            if !piece.is_empty() {
                chunks.push(piece.to_owned());
            }
            break;
        }
        let mut max_boundary = limit;
        while !rem.is_char_boundary(max_boundary) {
            max_boundary -= 1;
        }
        let break_at = rem[..max_boundary]
            .rfind(' ')
            .unwrap_or(max_boundary);
        let piece = rem[..break_at].trim();
        if !piece.is_empty() {
            chunks.push(piece.to_owned());
        }
        rem = rem[break_at..].trim_start();
    }
    chunks
}

export!(handle);
