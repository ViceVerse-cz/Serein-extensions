/// VenSerein (RPCEditor): Advanced RPC assets, timestamps, and buttons editor.
use serein_extension_sdk::{export, Element, Invocation, Output};

fn handle(inv: Invocation) -> Output {
    if inv.action == "rpc-assets" {
        Output {
            panel: vec![
                Element::Text { text: "RPC Assets & Timers".into() },
                Element::TextInput { id: "large_image".into(), label: "Large Image Key / URL".into(), value: String::new() },
                Element::TextInput { id: "small_image".into(), label: "Small Image Key / URL".into(), value: String::new() },
            ],
            ..Default::default()
        }
    } else {
        Output::default()
    }
}

export!(handle);
