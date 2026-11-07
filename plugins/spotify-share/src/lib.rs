use serein_extension_sdk::{export, Element, Invocation, Output};
use url::Url;

pub fn normalize_spotify_link(input: &str) -> Option<(String, String)> {
	let trimmed = input.trim();

	// Case 1: spotify:type:id URI
	if let Some(rest) = trimmed.strip_prefix("spotify:") {
		let parts: Vec<&str> = rest.split(':').collect();
		if parts.len() == 2 {
			let kind = parts[0];
			let id = parts[1];
			let url = format!("https://open.spotify.com/{kind}/{id}");
			return Some((kind.to_string(), url));
		}
	}

	// Case 2: https://open.spotify.com/type/id?...
	if let Ok(mut parsed) = Url::parse(trimmed) {
		if parsed.host_str() == Some("open.spotify.com") {
			let kind_opt = parsed
				.path_segments()
				.and_then(|mut s| s.next().map(|k| k.to_string()));

			if let Some(kind) = kind_opt {
				// Clean tracking queries (si, utm, context)
				parsed.set_query(None);
				return Some((kind, parsed.to_string()));
			}
		}
	}

	None
}

pub fn handle(input: Invocation) -> Output {
	let composer = input.composer.as_deref().unwrap_or("").trim();

	let mut panel = vec![Element::Heading {
		text: "Spotify Share Helper".into(),
	}];

	if composer.is_empty() {
		panel.push(Element::Text {
			text: "Enter a Spotify link (e.g. https://open.spotify.com/track/... or spotify:track:...) in your composer.".into(),
		});
		return Output {
			panel,
			..Default::default()
		};
	}

	let Some((kind, clean_url)) = normalize_spotify_link(composer) else {
		panel.push(Element::Text {
			text: format!("Could not recognize a valid Spotify URI or URL in composer:\n\"{composer}\""),
		});
		return Output {
			panel,
			..Default::default()
		};
	};

	let kind_cap = match kind.as_str() {
		"track" => "Track",
		"album" => "Album",
		"playlist" => "Playlist",
		"artist" => "Artist",
		"episode" => "Podcast Episode",
		_ => "Spotify Item",
	};

	let formatted_markdown = format!("🎵 **[{kind_cap}]({clean_url})**");

	panel.push(Element::Text {
		text: format!("Detected {kind_cap}: {clean_url}"),
	});
	panel.push(Element::Text {
		text: format!("Markdown Preview: {formatted_markdown}"),
	});

	if input.action == "format-spotify" || input.action == "apply-markdown" {
		Output {
			replacement: Some(formatted_markdown),
			panel,
			..Default::default()
		}
	} else {
		panel.push(Element::Button {
			id: "apply-markdown".into(),
			label: "Replace Composer with Clean Spotify Link".into(),
		});

		Output {
			panel,
			..Default::default()
		}
	}
}

export!(handle);

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn normalizes_spotify_uri_to_web_url() {
		let (kind, url) =
			normalize_spotify_link("spotify:track:4cOdK2wGLETKBW3PvgPWqT").expect("valid");
		assert_eq!(kind, "track");
		assert_eq!(url, "https://open.spotify.com/track/4cOdK2wGLETKBW3PvgPWqT");
	}

	#[test]
	fn strips_tracking_query_from_web_url() {
		let (kind, url) = normalize_spotify_link(
			"https://open.spotify.com/album/1DFixLWuPkv3KT3TnV35m3?si=abcdef123456&context=spotify",
		)
		.expect("valid");
		assert_eq!(kind, "album");
		assert_eq!(url, "https://open.spotify.com/album/1DFixLWuPkv3KT3TnV35m3");
	}

	#[test]
	fn formats_composer_with_markdown() {
		let input = Invocation {
			action: "format-spotify".into(),
			composer: Some("spotify:track:4cOdK2wGLETKBW3PvgPWqT".into()),
			..Default::default()
		};
		let output = handle(input);
		assert_eq!(
			output.replacement.as_deref(),
			Some("🎵 **[Track](https://open.spotify.com/track/4cOdK2wGLETKBW3PvgPWqT)**")
		);
	}
}
