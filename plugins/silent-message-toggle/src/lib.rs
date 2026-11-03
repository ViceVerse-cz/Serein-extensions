use serein_extension_sdk::{Element, Invocation, Output};

pub fn toggle_silent(draft: &str) -> (String, bool) {
    let trimmed = draft.trim_start();
    if let Some(rest) = trimmed.strip_prefix("@silent") {
        let clean = rest.trim_start();
        (clean.to_string(), false)
    } else {
        if draft.is_empty() {
            ("@silent ".to_string(), true)
        } else {
            (format!("@silent {draft}"), true)
        }
    }
}

pub fn handle(input: Invocation) -> Output {
    if input.action != "toggle-silent" {
        return Output::default();
    }

    let draft = input.composer.as_deref().unwrap_or("");
    let (new_draft, enabled) = toggle_silent(draft);

    let status_text = if enabled {
        "? Режим @silent включён (сообщение будет отправлено без звукового уведомления)."
    } else {
        "?? Режим @silent выключен."
    };

    Output {
        replacement: Some(new_draft),
        panel: vec![Element::Text {
            text: status_text.into(),
        }],
        ..Default::default()
    }
}

serein_extension_sdk::export!(handle);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_toggle_silent_on() {
        let draft = "Hello world";
        let (res, enabled) = toggle_silent(draft);
        assert!(enabled);
        assert_eq!(res, "@silent Hello world");
    }

    #[test]
    fn test_toggle_silent_off() {
        let draft = "@silent Hello world";
        let (res, enabled) = toggle_silent(draft);
        assert!(!enabled);
        assert_eq!(res, "Hello world");
    }

    #[test]
    fn test_toggle_silent_empty() {
        let draft = "";
        let (res, enabled) = toggle_silent(draft);
        assert!(enabled);
        assert_eq!(res, "@silent ");
    }
}
