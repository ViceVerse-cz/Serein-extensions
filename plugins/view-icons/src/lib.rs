use serein_extension_sdk::{AppInvocation, AppOutput, Element, HostEffect, Output};

pub fn build_icon_cdn_url(guild_id: &str, icon_hash: &str, format: &str) -> String {
    format!("https://cdn.discordapp.com/icons/{guild_id}/{icon_hash}.{format}?size=1024")
}

pub fn handle(input: AppInvocation) -> AppOutput {
    let inv = &input.invocation;

    let app = match input.app.as_ref() {
        Some(a) => a,
        None => {
            return AppOutput {
                output: Output {
                    panel: vec![Element::Text {
                        text: "Данные приложения недоступны (нет разрешения app_context).".into(),
                    }],
                    ..Default::default()
                },
                ..Default::default()
            };
        }
    };

    let cur_guild_id = app
        .context
        .as_ref()
        .and_then(|c| c.channel.as_ref())
        .and_then(|c| c.guild_id.as_deref());

    let Some(gid) = cur_guild_id else {
        return AppOutput {
            output: Output {
                panel: vec![Element::Text {
                    text: "Вы находитесь в личных сообщениях. Откройте сервер для просмотра иконки.".into(),
                }],
                ..Default::default()
            },
            ..Default::default()
        };
    };

    let guild = app
        .guilds
        .as_ref()
        .and_then(|g| g.items.iter().find(|i| i.id == gid));

    let gname = guild.map_or("Текущий сервер", |g| g.name.as_str());
    let icon_hash = guild.and_then(|g| g.icon.as_deref());

    match inv.action.as_str() {
        "copy-icon-url" => {
            if let Some(hash) = icon_hash {
                let ext = if hash.starts_with("a_") { "gif" } else { "png" };
                let url = build_icon_cdn_url(gid, hash, ext);
                AppOutput {
                    output: Output {
                        panel: vec![Element::Text {
                            text: format!("Ссылка на иконку скопирована: {url}"),
                        }],
                        ..Default::default()
                    },
                    effects: vec![HostEffect::CopyText { text: url }],
                }
            } else {
                AppOutput {
                    output: Output {
                        panel: vec![Element::Text {
                            text: "У данного сервера нет установленной иконки.".into(),
                        }],
                        ..Default::default()
                    },
                    ..Default::default()
                }
            }
        }
        _ => {
            // "show-assets"
            let mut panel = vec![
                Element::Heading {
                    text: format!("Ассеты сервера {gname}"),
                },
            ];

            if let Some(hash) = icon_hash {
                let is_animated = hash.starts_with("a_");
                let png_url = build_icon_cdn_url(gid, hash, "png");
                let webp_url = build_icon_cdn_url(gid, hash, "webp");

                panel.push(Element::Text {
                    text: format!("?? Хеш иконки: `{hash}` (Анимированная: {is_animated})"),
                });
                panel.push(Element::Separator);
                panel.push(Element::Text {
                    text: format!("• **PNG 1024px**: {png_url}"),
                });
                panel.push(Element::Text {
                    text: format!("• **WebP 1024px**: {webp_url}"),
                });

                if is_animated {
                    let gif_url = build_icon_cdn_url(gid, hash, "gif");
                    panel.push(Element::Text {
                        text: format!("• **GIF 1024px**: {gif_url}"),
                    });
                }

                panel.push(Element::Separator);
                panel.push(Element::Button {
                    id: "copy-icon-url".into(),
                    label: "Скопировать прямую ссылку на иконку".into(),
                });
            } else {
                panel.push(Element::Text {
                    text: "У этого сервера нет кастомной иконки (используется дефолтная аббревиатура).".into(),
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
    fn test_panel_button_id_valid() {
        assert!(is_valid_serein_id("copy-icon-url"));
    }

    #[test]
    fn test_build_icon_cdn_url() {
        let url = build_icon_cdn_url("123456", "abcdef", "png");
        assert_eq!(
            url,
            "https://cdn.discordapp.com/icons/123456/abcdef.png?size=1024"
        );
    }
}
