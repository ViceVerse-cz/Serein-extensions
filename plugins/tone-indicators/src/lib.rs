/// VenSerein (ToneIndicators): Show tooltips for tone indicators (/s, /j, /gen, /srs).
use serein_extension_sdk::{export, Invocation, Output, Element};

const TONES: &[(&str, &str)] = &[
    ("/s",   "Sarcastic"),
    ("/j",   "Joking"),
    ("/gen", "Genuine"),
    ("/srs", "Serious"),
    ("/hj",  "Half-joking"),
    ("/lh",  "Light-hearted"),
    ("/t",   "Teasing"),
    ("/nf",  "Not funny"),
    ("/p",   "Platonic"),
    ("/r",   "Romantic"),
    ("/nm",  "Not mad / upset"),
    ("/lu",  "A little upset"),
    ("/nbh", "Nobody here (venting)"),
    ("/nsb", "Not subtweeting"),
    ("/neg", "Negative"),
    ("/pos", "Positive"),
    ("/npa", "Non-passive-aggressive"),
    ("/ref", "Reference"),
    ("/th",  "Threat"),
    ("/cb",  "Clickbait"),
    ("/f",   "Fake"),
    ("/m",   "Mad / upset"),
    ("/li",  "Literally"),
    ("/hyp", "Hyperbole"),
    ("/ij",  "Inside joke"),
];

fn handle(inv: Invocation) -> Output {
    if inv.action == "toggle" {
        let list = TONES.iter()
            .map(|(k, v)| format!("{} = {}", k, v))
            .collect::<Vec<_>>()
            .join("  •  ");
        return Output {
            panel: vec![
                Element::Text { text: "Tone Indicators Reference".into() },
                Element::Text { text: list },
            ],
            ..Default::default()
        };
    }
    Output::default()
}

export!(handle);
