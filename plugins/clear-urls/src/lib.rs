use serein_extension_sdk::{
    AppInvocation, AppOutput, Element, HostEffect, Output,
};
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ClearUrlSettings {
    pub clean_utm: bool,
    pub clean_social_shares: bool,
    pub clean_ad_click_ids: bool,
}

impl Default for ClearUrlSettings {
    fn default() -> Self {
        Self {
            clean_utm: true,
            clean_social_shares: true,
            clean_ad_click_ids: true,
        }
    }
}

pub fn is_tracking_param(key: &str, settings: &ClearUrlSettings) -> bool {
    let lower = key.to_ascii_lowercase();

    if settings.clean_utm && lower.starts_with("utm_") {
        return true;
    }

    if settings.clean_ad_click_ids {
        match lower.as_str() {
            "fbclid" | "gclid" | "gclsrc" | "dclid" | "msclkid" | "yclid" | "mc_eid"
            | "mc_cid" | "zanpid" | "igshid" | "_hsenc" | "_hsmi" | "wbraid" | "gbraid"
            | "rdt_cid" => return true,
            _ => {}
        }
    }

    if settings.clean_social_shares {
        match lower.as_str() {
            "si" | "ref_src" | "ref_url" | "share_id" | "sender_device" | "is_from_webapp"
            | "_r" | "_t" | "spm_id_from" | "from_source" | "vd_source" => return true,
            // Twitter / X share tracking
            "s" => return true,
            _ => {}
        }
    }

    false
}

pub fn clean_url(raw_url: &str, settings: &ClearUrlSettings) -> Option<String> {
    let mut parsed = Url::parse(raw_url).ok()?;
    if parsed.query().is_none() {
        return None;
    }

    let pairs: Vec<(String, String)> = parsed
        .query_pairs()
        .filter(|(k, _)| !is_tracking_param(k, settings))
        .map(|(k, v)| (k.into_owned(), v.into_owned()))
        .collect();

    let old_query_count = parsed.query_pairs().count();
    if pairs.len() == old_query_count {
        return None; // No tracking parameters found
    }

    parsed.set_query(None);
    if !pairs.is_empty() {
        let mut serializer = parsed.query_pairs_mut();
        for (k, v) in pairs {
            serializer.append_pair(&k, &v);
        }
    }

    Some(parsed.to_string())
}

pub fn clean_text(text: &str, settings: &ClearUrlSettings) -> (String, usize) {
    let mut result = String::with_capacity(text.len());
    let mut cleaned_count = 0;
    let mut last_idx = 0;

    for (start, _) in text.match_indices("http://").chain(text.match_indices("https://")) {
        if start < last_idx {
            continue;
        }

        result.push_str(&text[last_idx..start]);

        // Find end of URL
        let candidate = &text[start..];
        let end_offset = candidate
            .find(|c: char| c.is_whitespace() || c == '>' || c == ')' || c == ']' || c == '"' || c == '\'')
            .unwrap_or(candidate.len());

        let mut raw_url = &candidate[..end_offset];
        // Strip trailing punctuation if not part of URL
        while raw_url.ends_with('.') || raw_url.ends_with(',') || raw_url.ends_with(';') {
            raw_url = &raw_url[..raw_url.len() - 1];
        }

        if let Some(cleaned) = clean_url(raw_url, settings) {
            result.push_str(&cleaned);
            cleaned_count += 1;
        } else {
            result.push_str(raw_url);
        }

        last_idx = start + raw_url.len();
    }

    if last_idx < text.len() {
        result.push_str(&text[last_idx..]);
    }

    (result, cleaned_count)
}

fn form(settings: &ClearUrlSettings, message: Option<&str>) -> Output {
    let mut panel = vec![
        Element::Heading {
            text: "ClearURLs Settings".into(),
        },
        Element::Checkbox {
            id: "clean-utm".into(),
            label: "Очищать UTM метки (utm_source, utm_medium, ...)".into(),
            checked: settings.clean_utm,
        },
        Element::Checkbox {
            id: "clean-social-shares".into(),
            label: "Очищать трекинг шаринга (YouTube si, Spotify si, X/Twitter s, ...)".into(),
            checked: settings.clean_social_shares,
        },
        Element::Checkbox {
            id: "clean-ad-click-ids".into(),
            label: "Очищать рекламные идентификаторы (fbclid, gclid, yclid, ...)".into(),
            checked: settings.clean_ad_click_ids,
        },
        Element::Button {
            id: "save-settings".into(),
            label: "Сохранить настройки".into(),
        },
    ];

    if let Some(msg) = message {
        panel.push(Element::Separator);
        panel.push(Element::Text { text: msg.into() });
    }

    Output {
        panel,
        ..Default::default()
    }
}

pub fn handle(input: AppInvocation) -> AppOutput {
    let inv = &input.invocation;
    let settings = inv
        .storage_json::<ClearUrlSettings>()
        .ok()
        .flatten()
        .unwrap_or_default();

    match inv.action.as_str() {
        "clean-draft" => {
            let draft = inv.composer.as_deref().unwrap_or("");
            let (cleaned, count) = clean_text(draft, &settings);
            if count > 0 {
                AppOutput {
                    output: Output {
                        replacement: Some(cleaned),
                        panel: vec![Element::Text {
                            text: format!("Очищено ссылок от трекеров: {count}"),
                        }],
                        ..Default::default()
                    },
                    ..Default::default()
                }
            } else {
                AppOutput {
                    output: Output {
                        panel: vec![Element::Text {
                            text: "В драфте не найдено ссылок с трекинг-параметрами.".into(),
                        }],
                        ..Default::default()
                    },
                    ..Default::default()
                }
            }
        }
        "clean-message" => {
            let msg = inv.selected_message.as_deref().unwrap_or("");
            let (cleaned, count) = clean_text(msg, &settings);
            if count > 0 {
                AppOutput {
                    output: Output {
                        panel: vec![
                            Element::Heading {
                                text: "Очищенное сообщение".into(),
                            },
                            Element::Text {
                                text: format!("Удалено трекинг-параметров в {count} ссылках:"),
                            },
                            Element::Text {
                                text: cleaned.clone(),
                            },
                        ],
                        ..Default::default()
                    },
                    effects: vec![HostEffect::CopyText { text: cleaned }],
                }
            } else {
                AppOutput {
                    output: Output {
                        panel: vec![Element::Text {
                            text: "В сообщении не обнаружено ссылок с трекерами.".into(),
                        }],
                        ..Default::default()
                    },
                    ..Default::default()
                }
            }
        }
        "open-settings" => AppOutput {
            output: form(&settings, None),
            ..Default::default()
        },
        "save-settings" => {
            let clean_utm = inv
                .parse_value::<bool>("clean-utm")
                .ok()
                .flatten()
                .unwrap_or(true);
            let clean_social_shares = inv
                .parse_value::<bool>("clean-social-shares")
                .ok()
                .flatten()
                .unwrap_or(true);
            let clean_ad_click_ids = inv
                .parse_value::<bool>("clean-ad-click-ids")
                .ok()
                .flatten()
                .unwrap_or(true);

            let new_settings = ClearUrlSettings {
                clean_utm,
                clean_social_shares,
                clean_ad_click_ids,
            };

            let mut output = form(&new_settings, Some("Настройки ClearURLs успешно сохранены."));
            let _ = output.set_storage_json(&new_settings);

            AppOutput {
                output,
                ..Default::default()
            }
        }
        _ => AppOutput::default(),
    }
}

serein_extension_sdk::export!(handle);

#[cfg(test)]
mod tests {
    use super::*;

    fn is_valid_serein_id(value: &str) -> bool {
        !value.is_empty()
            && value.len() <= 64
            && value
                .bytes()
                .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
            && value.as_bytes()[0].is_ascii_alphanumeric()
    }

    #[test]
    fn test_panel_element_ids_valid() {
        let settings = ClearUrlSettings::default();
        let out = form(&settings, None);
        for element in out.panel {
            match element {
                Element::Checkbox { id, label, .. }
                | Element::Button { id, label }
                | Element::TextInput { id, label, .. } => {
                    assert!(
                        is_valid_serein_id(&id),
                        "Element id '{}' does not satisfy Serein valid_id (no underscores, lowercase and hyphens only)",
                        id
                    );
                    assert!(!label.is_empty() && label.len() <= 128);
                }
                _ => {}
            }
        }
    }

    #[test]
    fn test_clean_youtube_url() {
        let settings = ClearUrlSettings::default();
        let raw = "Check this: https://youtu.be/dQw4w9WgXcQ?si=abcdef123456 & enjoy!";
        let (cleaned, count) = clean_text(raw, &settings);
        assert_eq!(count, 1);
        assert_eq!(cleaned, "Check this: https://youtu.be/dQw4w9WgXcQ & enjoy!");
    }

    #[test]
    fn test_clean_utm_params() {
        let settings = ClearUrlSettings::default();
        let raw = "https://example.com/article?utm_source=twitter&utm_medium=social&page=2";
        let (cleaned, count) = clean_text(raw, &settings);
        assert_eq!(count, 1);
        assert_eq!(cleaned, "https://example.com/article?page=2");
    }

    #[test]
    fn test_clean_spotify_url() {
        let settings = ClearUrlSettings::default();
        let raw = "https://open.spotify.com/track/4cOdK2wGLETKBW3PvgPWqT?si=912830918239";
        let (cleaned, count) = clean_text(raw, &settings);
        assert_eq!(count, 1);
        assert_eq!(cleaned, "https://open.spotify.com/track/4cOdK2wGLETKBW3PvgPWqT");
    }
}
