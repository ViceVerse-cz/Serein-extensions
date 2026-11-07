use serein_extension_sdk::{
	export, CustomRichPresence, Element, Invocation, Output, RichPresenceButton,
	RichPresenceImage, RichPresenceKind, RichPresenceOutput, RichPresenceTimer, RichPresenceUpdate,
};

pub fn handle(input: Invocation) -> RichPresenceOutput {
	let action = input.action.as_str();

	let track = input.value("track").unwrap_or("Starboy").trim();
	let artist = input.value("artist").unwrap_or("The Weeknd").trim();
	let album = input.value("album").unwrap_or("Starboy").trim();

	let track_slug = format!("{track} {artist}").replace(' ', "+");
	let spotify_url = format!("https://open.spotify.com/search/{track_slug}");
	let share_markdown = format!("🎵 **Now Playing on Spotify:** [{track} — {artist}]({spotify_url})");

	if action == "share-song" {
		return RichPresenceOutput {
			output: Output {
				replacement: Some(share_markdown),
				panel: vec![
					Element::Heading {
						text: "Spotify Track Shared".into(),
					},
					Element::Text {
						text: format!("Added \"{track} by {artist}\" to message composer."),
					},
				],
				..Default::default()
			},
			rich_presence: None,
		};
	}

	let rich_presence = if action == "toggle-presence" || action == "play" {
		Some(RichPresenceUpdate::Set {
			presence: Box::new(CustomRichPresence {
				application_id: "100000000000000002".into(),
				name: "Spotify".into(),
				kind: RichPresenceKind::Listening,
				stream_url: None,
				details: Some(track.to_string()),
				details_url: Some(spotify_url.clone()),
				state: Some(format!("by {artist}")),
				state_url: Some(spotify_url.clone()),
				large_image: Some(RichPresenceImage {
					key: "spotify_logo".into(),
					text: Some(album.to_string()),
					url: None,
				}),
				small_image: Some(RichPresenceImage {
					key: "play".into(),
					text: Some("Playing".into()),
					url: None,
				}),
				buttons: vec![RichPresenceButton {
					label: "Play on Spotify".into(),
					url: spotify_url.clone(),
				}],
				party: None,
				timer: RichPresenceTimer::Elapsed,
			}),
		})
	} else if action == "pause" {
		Some(RichPresenceUpdate::Clear)
	} else {
		None
	};

	let panel = vec![
		Element::Heading {
			text: "Spotify Playback Controls".into(),
		},
		Element::Text {
			text: format!("Track: **{track}**"),
		},
		Element::Text {
			text: format!("Artist: **{artist}** (Album: {album})"),
		},
		Element::Row {
			children: vec![
				Element::Button {
					id: "toggle-presence".into(),
					label: "▶ Listen / Broadcast".into(),
				},
				Element::Button {
					id: "pause".into(),
					label: "⏸ Pause Activity".into(),
				},
			],
		},
		Element::Separator,
		Element::Button {
			id: "share-song".into(),
			label: "Insert Track Link into Composer".into(),
		},
	];

	RichPresenceOutput {
		output: Output {
			panel,
			..Default::default()
		},
		rich_presence,
	}
}

export!(handle);

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn formats_share_song_replacement() {
		let input = Invocation {
			action: "share-song".into(),
			..Default::default()
		};
		let output = handle(input);
		assert!(output.output.replacement.is_some());
		let text = output.output.replacement.unwrap();
		assert!(text.contains("Starboy"));
		assert!(text.contains("The Weeknd"));
		assert!(text.contains("https://open.spotify.com/search/"));
	}

	#[test]
	fn emits_listening_rich_presence_update() {
		let input = Invocation {
			action: "toggle-presence".into(),
			..Default::default()
		};
		let output = handle(input);
		assert!(output.rich_presence.is_some());
		if let Some(RichPresenceUpdate::Set { presence }) = output.rich_presence {
			assert_eq!(presence.name, "Spotify");
			assert_eq!(presence.kind, RichPresenceKind::Listening);
			assert_eq!(presence.details.as_deref(), Some("Starboy"));
		} else {
			panic!("Expected RichPresenceUpdate::Set");
		}
	}

	#[test]
	fn renders_playback_control_panel() {
		let input = Invocation {
			action: "control-panel".into(),
			..Default::default()
		};
		let output = handle(input);
		assert!(!output.output.panel.is_empty());
		assert!(output.rich_presence.is_none());
	}
}
