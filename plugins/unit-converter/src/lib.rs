/// VenSerein (UnitConverter): Auto-convert imperial/metric units in messages.
use serein_extension_sdk::{export, Invocation, Output, Element};

fn handle(inv: Invocation) -> Output {
    match inv.action.as_str() {
        "convert" => {
            let text = inv.composer.as_deref().unwrap_or("");
            Output { replacement: Some(convert_units(text)), ..Default::default() }
        }
        "settings" => Output {
            panel: vec![
                Element::Text { text: "Unit Converter Settings".into() },
                Element::Text { text: "Automatically converts: mi↔km, °F↔°C, lbs↔kg, ft↔m".into() },
            ],
            ..Default::default()
        },
        _ => Output::default(),
    }
}

fn convert_units(text: &str) -> String {
    // Simple pattern scan for common units
    let mut out = text.to_owned();
    // This is a scaffold; real impl would use regex captures.
    // We append conversion hints in parens.
    out = out.replace(" miles", " miles (km: use ×1.609)")
             .replace(" mph",   " mph (km/h: use ×1.609)")
             .replace("°F",     "°F (°C: subtract 32 then ÷1.8)")
             .replace(" lbs",   " lbs (kg: use ÷2.205)")
             .replace(" lb ",   " lb (kg: use ÷2.205)")
             .replace(" feet",  " feet (m: use ×0.305)")
             .replace(" ft ",   " ft (m: use ×0.305)");
    out
}

export!(handle);
