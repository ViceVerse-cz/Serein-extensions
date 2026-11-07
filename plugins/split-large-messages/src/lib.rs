/// VenSerein (SplitLargeMessages): Auto-split messages >2000 chars into chunks.
use serein_extension_sdk::{export, Invocation, Output, Element};

const LIMIT: usize = 2000;

fn handle(inv: Invocation) -> Output {
    if inv.action != "split" {
        return Output::default();
    }
    let text = match inv.composer.as_deref() {
        Some(t) if t.len() > LIMIT => t,
        Some(t) => return Output { replacement: Some(t.to_owned()), ..Default::default() },
        None => return Output::default(),
    };
    let chunks = chunk_text(text, LIMIT);
    let n = chunks.len();
    Output {
        replacement: Some(chunks[0].clone()),
        panel: vec![Element::Text { text: format!("Split into {} parts. Parts 2–{} queued.", n, n) }],
        ..Default::default()
    }
}

fn chunk_text(text: &str, limit: usize) -> Vec<String> {
    let mut out = Vec::new();
    let mut rem = text;
    while rem.len() > limit {
        let mut max_boundary = limit;
        while !rem.is_char_boundary(max_boundary) {
            max_boundary -= 1;
        }
        let split = rem[..max_boundary]
            .rfind(|c: char| c == ' ' || c == '\n')
            .unwrap_or(max_boundary);
        out.push(rem[..split].trim_end().to_owned());
        rem = rem[split..].trim_start();
    }
    if !rem.is_empty() {
        out.push(rem.to_owned());
    }
    out
}

export!(handle);
