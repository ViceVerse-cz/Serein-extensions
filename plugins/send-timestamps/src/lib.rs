use serein_extension_sdk::{AppInvocation, AppOutput, Element, Output};

const DISCORD_EPOCH_MS: u64 = 1_420_070_400_000;

pub fn snowflake_to_unix_secs(snowflake: &str) -> Option<u64> {
    let id: u64 = snowflake.parse().ok()?;
    let ms = (id >> 22) + DISCORD_EPOCH_MS;
    Some(ms / 1000)
}

/// Simple algorithm to convert year, month (1..=12), day (1..=31) to days since Unix epoch (1970-01-01).
pub fn days_from_civil(mut y: i64, m: u32, d: u32) -> i64 {
    y -= if m <= 2 { 1 } else { 0 };
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = (y - era * 400) as u32;
    let doy = (153 * (if m > 2 { m - 3 } else { m + 9 }) + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146097 + (doe as i64) - 719468
}

/// Parses "YYYY-MM-DD HH:MM" into Unix timestamp seconds.
pub fn parse_date_string(s: &str) -> Option<u64> {
    let s = s.trim();
    // Can be a raw unix timestamp number
    if let Ok(ts) = s.parse::<u64>() {
        return Some(ts);
    }

    let (date_part, time_part) = s.split_once(' ')?;
    let mut date_splits = date_part.split('-');
    let year: i64 = date_splits.next()?.parse().ok()?;
    let month: u32 = date_splits.next()?.parse().ok()?;
    let day: u32 = date_splits.next()?.parse().ok()?;

    if !(1..=12).contains(&month) || !(1..=31).contains(&day) || year < 1970 || year > 2100 {
        return None;
    }

    let mut time_splits = time_part.split(':');
    let hour: u64 = time_splits.next()?.parse().ok()?;
    let min: u64 = time_splits.next()?.parse().ok()?;
    let sec: u64 = time_splits.next().and_then(|s| s.parse().ok()).unwrap_or(0);

    if hour > 23 || min > 59 || sec > 59 {
        return None;
    }

    let days = days_from_civil(year, month, day);
    if days < 0 {
        return None;
    }

    let total_secs = (days as u64) * 86400 + hour * 3600 + min * 60 + sec;
    Some(total_secs)
}

pub fn parse_offset_minutes(preset: &str) -> u64 {
    if let Some((num, _)) = preset.split_once(':') {
        let clean = num.trim_start_matches('+').trim_end_matches('m');
        clean.parse().unwrap_or(0)
    } else {
        0
    }
}

pub fn parse_format_flag(style: &str) -> &str {
    if let Some((flag, _)) = style.split_once(':') {
        flag.trim()
    } else {
        "R"
    }
}

fn build_picker_panel(default_ts_str: &str, error_msg: Option<&str>) -> Output {
    let mut panel = vec![
        Element::Heading {
            text: "Discord Timestamp Picker".into(),
        },
        Element::Text {
            text: "Введите Unix timestamp или дату в формате YYYY-MM-DD HH:MM:".into(),
        },
        Element::TextInput {
            id: "timestamp-input".into(),
            label: "Метка времени".into(),
            value: default_ts_str.into(),
        },
        Element::Select {
            id: "offset-preset".into(),
            label: "Смещение времени".into(),
            options: vec![
                "+0m: Без смещения".into(),
                "+15m: +15 минут".into(),
                "+60m: +1 час".into(),
                "+180m: +3 часа".into(),
                "+1440m: +1 день".into(),
                "+10080m: +1 неделя".into(),
            ],
            value: "+0m: Без смещения".into(),
        },
        Element::Select {
            id: "format-style".into(),
            label: "Стиль отображения".into(),
            options: vec![
                "R: Относительное время (через 15 минут)".into(),
                "F: Полная дата и время (вторник, 3 ноября 2026 16:20)".into(),
                "f: Краткая дата и время (3 ноября 2026 16:20)".into(),
                "d: Краткая дата (03.11.2026)".into(),
                "D: Полная дата (3 ноября 2026)".into(),
                "t: Краткое время (16:20)".into(),
                "T: Полное время (16:20:00)".into(),
            ],
            value: "R: Относительное время (через 15 минут)".into(),
        },
        Element::Button {
            id: "apply-timestamp".into(),
            label: "Вставить метку в драфт".into(),
        },
    ];

    if let Some(err) = error_msg {
        panel.push(Element::Separator);
        panel.push(Element::Text { text: err.into() });
    }

    Output {
        panel,
        ..Default::default()
    }
}

pub fn handle(input: AppInvocation) -> AppOutput {
    let inv = &input.invocation;

    // Try to get base timestamp from last message in timeline as fallback
    let default_ts = input
        .app
        .as_ref()
        .and_then(|a| a.timeline.as_ref())
        .and_then(|t| t.messages.last())
        .and_then(|m| snowflake_to_unix_secs(&m.id))
        .unwrap_or(1_793_660_000); // 2026 default baseline fallback

    match inv.action.as_str() {
        "show-picker" => AppOutput {
            output: build_picker_panel(&default_ts.to_string(), None),
            ..Default::default()
        },
        "apply-timestamp" => {
            let input_val = inv.value("timestamp-input").unwrap_or("").trim();
            let base_ts = if input_val.is_empty() {
                Some(default_ts)
            } else {
                parse_date_string(input_val)
            };

            let Some(ts) = base_ts else {
                return AppOutput {
                    output: build_picker_panel(
                        input_val,
                        Some("Неверный формат даты/числа. Пример: 2026-11-03 16:20 или 1793660000"),
                    ),
                    ..Default::default()
                };
            };

            let offset_preset = inv.value("offset-preset").unwrap_or("+0m: Без смещения");
            let offset_secs = parse_offset_minutes(offset_preset) * 60;
            let final_ts = ts + offset_secs;

            let format_style = inv.value("format-style").unwrap_or("R: Относительное время");
            let flag = parse_format_flag(format_style);

            let tag = format!("<t:{final_ts}:{flag}>");
            let draft = inv.composer.as_deref().unwrap_or("");
            let new_draft = if draft.is_empty() {
                tag
            } else {
                format!("{draft} {tag}")
            };

            AppOutput {
                output: Output {
                    replacement: Some(new_draft),
                    panel: vec![Element::Text {
                        text: format!("Вставлена метка: <t:{final_ts}:{flag}>"),
                    }],
                    ..Default::default()
                },
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
        let out = build_picker_panel("1793660000", None);
        for element in out.panel {
            match element {
                Element::Button { id, label }
                | Element::Select { id, label, .. }
                | Element::TextInput { id, label, .. } => {
                    assert!(
                        is_valid_serein_id(&id),
                        "Element id '{}' does not satisfy Serein valid_id",
                        id
                    );
                    assert!(!label.is_empty() && label.len() <= 128);
                }
                _ => {}
            }
        }
    }

    #[test]
    fn test_parse_date_string() {
        let parsed = parse_date_string("2026-11-03 16:20").unwrap();
        assert!(parsed > 1_700_000_000);
    }

    #[test]
    fn test_snowflake_to_timestamp() {
        // Discord snowflake format
        let sf = "1000000000000000000";
        let ts = snowflake_to_unix_secs(sf).unwrap();
        assert!(ts > 1_650_000_000);
    }
}
