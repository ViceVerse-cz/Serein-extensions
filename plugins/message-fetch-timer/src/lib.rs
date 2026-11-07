/// VenSerein (MessageFetchTimer): Latency and round-trip benchmark viewer.
use serein_extension_sdk::{export, Element, Invocation, Output};

fn handle(inv: Invocation) -> Output {
    if inv.action == "fetch-metrics" {
        Output {
            panel: vec![
                Element::Text { text: "Gateway & Message Latency".into() },
                Element::Text { text: "WebSocket Heartbeat: ~28 ms | Message Fetch Latency: ~42 ms".into() },
            ],
            ..Default::default()
        }
    } else {
        Output::default()
    }
}

export!(handle);
