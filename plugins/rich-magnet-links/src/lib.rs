/// VenSerein (RichMagnetLinks): Parse magnet links and render torrent metadata inline.
use serein_extension_sdk::{export, Invocation, Output, Element};

fn handle(inv: Invocation) -> Output {
    if inv.action != "expand-magnet" {
        return Output::default();
    }
    let text = inv.composer.as_deref().unwrap_or("");
    let magnet = text.split_whitespace().find(|s| s.starts_with("magnet:?")).unwrap_or(text);
    let info = parse_magnet(magnet);
    Output {
        panel: vec![
            Element::Text { text: format!("🧲 {}", info.name) },
            Element::Text { text: format!("Size: {}", info.size) },
            Element::Text { text: format!("Trackers: {}", info.trackers.join(", ")) },
        ],
        ..Default::default()
    }
}

struct MagnetInfo {
    name: String,
    size: String,
    trackers: Vec<String>,
}

fn parse_magnet(magnet: &str) -> MagnetInfo {
    let mut name = String::from("Unknown");
    let mut size = String::from("Unknown");
    let mut trackers = Vec::new();
    for part in magnet.trim_start_matches("magnet:?").split('&') {
        if let Some(val) = part.strip_prefix("dn=") {
            name = percent_decode(val);
        } else if let Some(val) = part.strip_prefix("xl=") {
            let bytes: u64 = val.parse().unwrap_or(0);
            size = format!("{:.1} MB", bytes as f64 / 1_048_576.0);
        } else if let Some(val) = part.strip_prefix("tr=") {
            trackers.push(percent_decode(val));
        }
    }
    if trackers.is_empty() {
        trackers.push("No trackers".into());
    }
    MagnetInfo { name, size, trackers }
}

fn percent_decode(s: &str) -> String {
    s.replace("%20", " ").replace("%3A", ":").replace("%2F", "/").replace('+', " ")
}

export!(handle);
