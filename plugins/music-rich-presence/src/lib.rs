use serein_extension_sdk::{
	export, CustomRichPresence, Element, Invocation, Output, RichPresenceButton,
	RichPresenceImage, RichPresenceKind, RichPresenceOutput, RichPresenceTimer, RichPresenceUpdate,
};

pub fn handle(input: Invocation) -> RichPresenceOutput {
	let action = input.action.as_str();

	if action == "clear" {
		return RichPresenceOutput {
			output: Output {
				panel: vec![
					Element::Heading {
						text: "Music Status Cleared".into(),
					},
					Element::Text {
						text: "Your listening activity has been removed from Discord Rich Presence."
							.into(),
					},
					Element::Button {
						id: "open-player".into(),
						label: "Back to Music Player".into(),
					},
				],
				..Default::default()
			},
			rich_presence: Some(RichPresenceUpdate::Clear),
		};
	}

	let track = input.value("track").unwrap_or("Blinding Lights").trim();
	let artist = input.value("artist").unwrap_or("The Weeknd").trim();
	let album = input.value("album").unwrap_or("After Hours").trim();

	let mut rich_presence = None;

	if action == "set-presence" || action == "play-default" {
		let presence = CustomRichPresence {
			application_id: "100000000000000001".into(),
			name: "Music".into(),
			kind: RichPresenceKind::Listening,
			stream_url: None,
			details: Some(track.to_string()),
			details_url: None,
			state: Some(format!("by {artist}")),
			state_url: None,
			large_image: Some(RichPresenceImage {
				key: "music_cover".into(),
				text: Some(format!("{album} by {artist}")),
				url: None,
			}),
			small_image: Some(RichPresenceImage {
				key: "play_icon".into(),
				text: Some("Playing".into()),
				url: None,
			}),
			buttons: vec![RichPresenceButton {
				label: "Listen Along".into(),
				url: "https://open.spotify.com".into(),
			}],
			party: None,
			timer: RichPresenceTimer::Elapsed,
		};
		rich_presence = Some(RichPresenceUpdate::Set {
			presence: Box::new(presence),
		});
	}

	let panel = vec![
		Element::Heading {
			text: "Music Rich Presence Controller".into(),
		},
		Element::Text {
			text: format!("Current Status: Listening to \"{track}\" by {artist} ({album})"),
		},
		Element::Separator,
		Element::TextInput {
			id: "track".into(),
			label: "Track Title".into(),
			value: track.to_string(),
		},
		Element::TextInput {
			id: "artist".into(),
			label: "Artist Name".into(),
			value: artist.to_string(),
		},
		Element::TextInput {
			id: "album".into(),
			label: "Album Name".into(),
			value: album.to_string(),
		},
		Element::Row {
			children: vec![
				Element::Button {
					id: "set-presence".into(),
					label: "Broadcast Music Status".into(),
				},
				Element::Button {
					id: "clear".into(),
					label: "Clear Activity".into(),
				},
			],
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
	fn sets_music_listening_presence() {
		let input = Invocation {
			action: "set-presence".into(),
			..Default::default()
		};
		let output = handle(input);
		assert!(output.rich_presence.is_some());
		match output.rich_presence.unwrap() {
			RichPresenceUpdate::Set { presence } => {
				assert_eq!(presence.name, "Music");
				assert_eq!(presence.kind, RichPresenceKind::Listening);
				assert_eq!(presence.details.as_deref(), Some("Blinding Lights"));
				assert_eq!(presence.state.as_deref(), Some("by The Weeknd"));
			}
			RichPresenceUpdate::Clear => panic!("expected Set"),
		}
	}

	#[test]
	fn clears_music_presence() {
		let input = Invocation {
			action: "clear".into(),
			..Default::default()
		};
		let output = handle(input);
		assert_eq!(output.rich_presence, Some(RichPresenceUpdate::Clear));
	}
}
