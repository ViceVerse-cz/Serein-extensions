use serein_extension_sdk::{AppInvocation, AppOutput, Element, HostEffect, Output};

const DISCORD_EPOCH_MS: u64 = 1_420_070_400_000;

pub fn snowflake_to_unix_secs(snowflake: &str) -> Option<u64> {
    let id: u64 = snowflake.parse().ok()?;
    let ms = (id >> 22) + DISCORD_EPOCH_MS;
    Some(ms / 1000)
}

/// Converts Unix timestamp seconds to a simple readable UTC string "YYYY-MM-DD HH:MM:SS".
pub fn format_unix_timestamp(ts_secs: u64) -> String {
    let sec = ts_secs % 60;
    let total_mins = ts_secs / 60;
    let min = total_mins % 60;
    let total_hours = total_mins / 60;
    let hour = total_hours % 24;
    let mut days = (total_hours / 24) as i64;

    // Civil day algorithm
    days += 719468;
    let era = if days >= 0 { days } else { days - 146096 } / 146097;
    let doe = (days - era * 146097) as u32;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = y + if m <= 2 { 1 } else { 0 };

    format!("{y:04}-{m:02}-{d:02} {hour:02}:{min:02}:{sec:02} UTC")
}

#[derive(Default)]
pub struct ChannelStats {
    pub text: usize,
    pub voice: usize,
    pub category: usize,
    pub other: usize,
    pub total: usize,
}

pub fn count_channels(channels: &[serein_extension_sdk::ChannelSnapshot], guild_id: Option<&str>) -> ChannelStats {
    let mut stats = ChannelStats::default();
    for c in channels {
        if guild_id.is_some() && c.guild_id.as_deref() != guild_id {
            continue;
        }
        stats.total += 1;
        match c.kind {
            0 => stats.text += 1,
            2 => stats.voice += 1,
            4 => stats.category += 1,
            _ => stats.other += 1,
        }
    }
    stats
}

pub fn handle(input: AppInvocation) -> AppOutput {
    let inv = &input.invocation;

    let app = match input.app.as_ref() {
        Some(a) => a,
        None => {
            return AppOutput {
                output: Output {
                    panel: vec![Element::Text {
                        text: "Данные приложения недоступны (нет прав app_context).".into(),
                    }],
                    ..Default::default()
                },
                ..Default::default()
            };
        }
    };

    let cur_channel = app.context.as_ref().and_then(|c| c.channel.as_ref());
    let cur_guild_id = cur_channel.and_then(|c| c.guild_id.as_deref());

    match inv.action.as_str() {
        "copy-guild-id" => {
            let gid = cur_guild_id.unwrap_or("DM");
            AppOutput {
                output: Output {
                    panel: vec![Element::Text {
                        text: format!("Server ID скопирован: {gid}"),
                    }],
                    ..Default::default()
                },
                effects: vec![HostEffect::CopyText { text: gid.to_string() }],
            }
        }
        "copy-channel-id" => {
            let cid = cur_channel.map_or("None", |c| &c.id);
            AppOutput {
                output: Output {
                    panel: vec![Element::Text {
                        text: format!("Channel ID скопирован: {cid}"),
                    }],
                    ..Default::default()
                },
                effects: vec![HostEffect::CopyText { text: cid.to_string() }],
            }
        }
        _ => {
            // "inspect-server" or default
            let mut panel = vec![
                Element::Heading {
                    text: "Техническая информация о сервере".into(),
                },
            ];

            if let Some(gid) = cur_guild_id {
                let guild = app.guilds.as_ref().and_then(|g| g.items.iter().find(|item| item.id == gid));
                let gname = guild.map_or("Неизвестный сервер", |g| g.name.as_str());

                let created_str = snowflake_to_unix_secs(gid)
                    .map(format_unix_timestamp)
                    .unwrap_or_else(|| "Неизвестно".into());

                panel.push(Element::Text {
                    text: format!("?? Сервер: **{gname}**"),
                });
                panel.push(Element::Text {
                    text: format!("?? Server ID: `{gid}`"),
                });
                panel.push(Element::Text {
                    text: format!("?? Дата создания (Snowflake): {created_str}"),
                });

                if let Some(channels) = app.channels.as_ref() {
                    let stats = count_channels(&channels.items, Some(gid));
                    panel.push(Element::Separator);
                    panel.push(Element::Heading {
                        text: "Статистика каналов".into(),
                    });
                    panel.push(Element::Text {
                        text: format!(
                            "?? Всего каналов: {} (Текстовых: {}, Голосовых: {}, Категорий: {}, Прочих: {})",
                            stats.total, stats.text, stats.voice, stats.category, stats.other
                        ),
                    });
                }

                if let Some(members) = app.members.as_ref() {
                    panel.push(Element::Text {
                        text: format!("?? Загружено участников: {}", members.items.len()),
                    });
                }

                panel.push(Element::Separator);
                panel.push(Element::Button {
                    id: "copy-guild-id".into(),
                    label: "Скопировать Server ID".into(),
                });
            } else {
                panel.push(Element::Text {
                    text: "Текущий контекст: Личные сообщения (DM / Direct Messages).".into(),
                });
            }

            if let Some(channel) = cur_channel {
                panel.push(Element::Separator);
                panel.push(Element::Heading {
                    text: "Текущий канал".into(),
                });
                panel.push(Element::Text {
                    text: format!("?? Имя: **#{}** | ID: `{}`", channel.name, channel.id),
                });
                panel.push(Element::Button {
                    id: "copy-channel-id".into(),
                    label: "Скопировать Channel ID".into(),
                });
            }

            AppOutput {
                output: Output {
                    panel,
                    ..Default::default()
                },
                ..Default::default()
            }
        }
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
    fn test_panel_buttons_valid() {
        let buttons = ["copy-guild-id", "copy-channel-id"];
        for id in buttons {
            assert!(is_valid_serein_id(id), "Button ID '{}' invalid", id);
        }
    }

    #[test]
    fn test_format_unix_timestamp() {
        // 1793660000 = 2026-11-02 22:53:20 UTC
        let formatted = format_unix_timestamp(1_793_660_000);
        assert!(formatted.contains("2026-11-02"));
    }

    #[test]
    fn test_snowflake_conversion() {
        let sf = "1000000000000000000";
        let secs = snowflake_to_unix_secs(sf).unwrap();
        let formatted = format_unix_timestamp(secs);
        assert!(formatted.contains("2022"));
    }
}
