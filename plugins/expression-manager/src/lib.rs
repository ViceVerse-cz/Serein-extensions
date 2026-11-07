use serein_extension_sdk::{export, Element, Invocation, Output};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct FavoriteEntry {
	pub name: String,
	pub content: String,
	pub category: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExpressionsStore {
	pub favorites: Vec<FavoriteEntry>,
	pub auto_expand_shortcodes: bool,
	pub active_category: String,
}

impl Default for ExpressionsStore {
	fn default() -> Self {
		Self {
			favorites: vec![
				FavoriteEntry {
					name: "shrug".into(),
					content: "¯\\_(ツ)_/¯".into(),
					category: "shrug".into(),
				},
				FavoriteEntry {
					name: "tableflip".into(),
					content: "(╯°□°)╯︵ ┻━┻".into(),
					category: "flip".into(),
				},
				FavoriteEntry {
					name: "unflip".into(),
					content: "┬─┬ノ( º _ ºノ)".into(),
					category: "flip".into(),
				},
				FavoriteEntry {
					name: "lenny".into(),
					content: "( ͡° ͜ʖ ͡°)".into(),
					category: "cool".into(),
				},
				FavoriteEntry {
					name: "dealwithit".into(),
					content: "(⌐■_■)".into(),
					category: "cool".into(),
				},
				FavoriteEntry {
					name: "hug".into(),
					content: "(づ｡◕‿‿◕｡)づ".into(),
					category: "love".into(),
				},
				FavoriteEntry {
					name: "cry".into(),
					content: "(ಥ﹏ಥ)".into(),
					category: "sad".into(),
				},
			],
			auto_expand_shortcodes: true,
			active_category: "all".into(),
		}
	}
}

pub const BUILTIN_KAOMOJI: &[(&str, &str, &str)] = &[
	("joy-1", "(・∀・)", "joy"),
	("joy-2", "(≧◡≦)", "joy"),
	("joy-3", "(✿◠‿◠)", "joy"),
	("joy-4", "(* ^ ω ^)", "joy"),
	("shrug-1", "¯\\_(ツ)_/¯", "shrug"),
	("shrug-2", "¯\\(°_o)/¯", "shrug"),
	("shrug-3", "¯\\_( ͡° ͜ʖ ͡°)_/¯", "shrug"),
	("flip-1", "(╯°□°)╯︵ ┻━┻", "flip"),
	("flip-2", "┬─┬ノ( º _ ºノ)", "flip"),
	("flip-3", "(╯°Д°)╯︵/(.□ . \\)", "flip"),
	("love-1", "(♡-_-♡)", "love"),
	("love-2", "(づ￣ ³￣)づ", "love"),
	("love-3", "(っ˘з(˘⌣˘ )", "love"),
	("love-4", "(づ｡◕‿‿◕｡)づ", "love"),
	("cool-1", "( ͡° ͜ʖ ͡°)", "cool"),
	("cool-2", "(▀̿Ĺ̯▀̿ ̿)", "cool"),
	("cool-3", "(⌐■_■)", "cool"),
	("sad-1", "(ಥ﹏ಥ)", "sad"),
	("sad-2", "(╥﹏╥)", "sad"),
	("sad-3", "( ; ω ; )", "sad"),
	("shock-1", "(ಠ_ಠ)", "shock"),
	("shock-2", "(⊙_⊙)", "shock"),
	("shock-3", "(╬ Ò﹏Ó)", "shock"),
];

pub fn parse_store(storage: Option<&str>) -> ExpressionsStore {
	storage
		.and_then(|s| serde_json::from_str::<ExpressionsStore>(s).ok())
		.unwrap_or_default()
}

pub fn expand_shortcodes(text: &str, store: &ExpressionsStore) -> String {
	let mut result = text.to_owned();
	for fav in &store.favorites {
		let code = format!(":{}:", fav.name);
		if result.contains(&code) {
			result = result.replace(&code, &fav.content);
		}
	}
	// Common built-in aliases
	let standard_aliases = [
		(":shrug:", "¯\\_(ツ)_/¯"),
		(":tableflip:", "(╯°□°)╯︵ ┻━┻"),
		(":unflip:", "┬─┬ノ( º _ ºノ)"),
		(":lenny:", "( ͡° ͜ʖ ͡°)"),
		(":dealwithit:", "(⌐■_■)"),
		(":hug:", "(づ｡◕‿‿◕｡)づ"),
		(":cry:", "(ಥ﹏ಥ)"),
		(":disapproval:", "(ಠ_ಠ)"),
		(":flip:", "(╯°□°)╯︵ ┻━┻"),
	];
	for (code, repl) in standard_aliases {
		if result.contains(code) {
			result = result.replace(code, repl);
		}
	}
	result
}

pub fn handle_slash_command(cmd: &str, store: &mut ExpressionsStore) -> (Option<String>, bool) {
	let parts: Vec<&str> = cmd.split_whitespace().collect();
	if parts.is_empty() {
		return (None, false);
	}

	let root = parts[0];
	if root == "/kaomoji" {
		let query = if parts.len() > 1 { parts[1] } else { "" };
		if query.is_empty() {
			return (Some("¯\\_(ツ)_/¯".into()), false);
		}
		for &(name, text, cat) in BUILTIN_KAOMOJI {
			if name.contains(query) || cat.contains(query) {
				return (Some(text.to_owned()), false);
			}
		}
		return (Some(format!("(Kaomoji matching '{}' not found)", query)), false);
	}

	if root != "/expressions" && root != "/expr" {
		return (None, false);
	}

	let sub = if parts.len() > 1 { parts[1] } else { "help" };

	match sub {
		"help" => {
			let help = "✨ **VenSerein Expression Manager**\n\
			• `/expressions list` — Show all kaomoji expressions\n\
			• `/expressions favs` — Show saved favorites\n\
			• `/expressions fav <name> <content>` — Pin expression to favorites\n\
			• `/expressions unfav <name>` — Remove favorite\n\
			• `/expressions expand <text>` — Expand shortcodes like `:shrug:`\n\
			• `/kaomoji <category>` — Quick search (joy, shrug, flip, love, cool, sad, shock)";
			(Some(help.into()), false)
		}
		"list" => {
			let mut out = String::from("📑 **Expression Library**\n\n");
			for &(name, text, cat) in BUILTIN_KAOMOJI {
				out.push_str(&format!("• `{}` [{}] — {}\n", name, cat, text));
			}
			(Some(out), false)
		}
		"favs" => {
			let mut out = String::from("⭐ **Saved Favorites**\n\n");
			for f in &store.favorites {
				out.push_str(&format!("• `:{}:` [{}] — {}\n", f.name, f.category, f.content));
			}
			(Some(out), false)
		}
		"fav" => {
			if parts.len() >= 4 {
				let name = parts[2].trim().trim_matches(':').to_lowercase();
				let content = parts[3..].join(" ");
				store.favorites.retain(|f| f.name != name);
				if store.favorites.len() >= 100 {
					store.favorites.remove(0);
				}
				store.favorites.push(FavoriteEntry {
					name: name.clone(),
					content: content.clone(),
					category: "custom".into(),
				});
				(Some(format!("Pinned favorite `:{}:` -> {}", name, content)), true)
			} else {
				(Some("Usage: `/expressions fav <name> <content>`".into()), false)
			}
		}
		"unfav" => {
			if parts.len() >= 3 {
				let name = parts[2].trim().trim_matches(':').to_lowercase();
				let before = store.favorites.len();
				store.favorites.retain(|f| f.name != name);
				let removed = store.favorites.len() < before;
				if removed {
					(Some(format!("Removed favorite `:{}:`", name)), true)
				} else {
					(Some(format!("Favorite `:{}:` not found", name)), false)
				}
			} else {
				(Some("Usage: `/expressions unfav <name>`".into()), false)
			}
		}
		"expand" => {
			if parts.len() > 2 {
				let text = parts[2..].join(" ");
				(Some(expand_shortcodes(&text, store)), false)
			} else {
				(Some("Usage: `/expressions expand <text>`".into()), false)
			}
		}
		_ => (Some("Unknown sub-command. Use `/expressions help`.".into()), false),
	}
}

pub fn handle(input: Invocation) -> Output {
	let mut store = parse_store(input.storage.as_deref());
	let action = input.action.as_str();
	let mut storage_changed = false;

	// Composer / slash command expansion
	if action == "expand-command" || action == "expand-expressions" {
		let composer_text = input.composer.as_deref().unwrap_or_default().trim();
		if composer_text.starts_with('/') {
			let (repl, changed) = handle_slash_command(composer_text, &mut store);
			if changed {
				storage_changed = true;
			}
			if let Some(r) = repl {
				let storage = if storage_changed {
					serde_json::to_string(&store).ok()
				} else {
					None
				};
				return Output {
					replacement: Some(r),
					panel: vec![
						Element::Heading {
							text: "Expression Manager".into(),
						},
						Element::Text {
							text: "Expression command processed.".into(),
						},
					],
					storage,
					..Default::default()
				};
			}
		}

		// Auto expand shortcodes if composer text contains known tags
		if store.auto_expand_shortcodes && !composer_text.is_empty() {
			let expanded = expand_shortcodes(composer_text, &store);
			if expanded != composer_text {
				return Output {
					replacement: Some(expanded),
					..Default::default()
				};
			}
		}
	}

	// Panel button actions
	if action == "toggle-expand" {
		store.auto_expand_shortcodes = !store.auto_expand_shortcodes;
		storage_changed = true;
	} else if action.starts_with("filter-") {
		store.active_category = action.trim_start_matches("filter-").to_owned();
		storage_changed = true;
	}

	let replacement = match action {
		"insert-shrug" => Some("¯\\_(ツ)_/¯".into()),
		"insert-flip" => Some("(╯°□°)╯︵ ┻━┻".into()),
		"insert-unflip" => Some("┬─┬ノ( º _ ºノ)".into()),
		"insert-lenny" => Some("( ͡° ͜ʖ ͡°)".into()),
		"insert-dealwithit" => Some("(⌐■_■)".into()),
		"insert-hug" => Some("(づ｡◕‿‿◕｡)づ".into()),
		"insert-cry" => Some("(ಥ﹏ಥ)".into()),
		_ => None,
	};

	let cat = store.active_category.as_str();

	let mut panel = vec![
		Element::Heading {
			text: "Expression Manager & Kaomoji Hub".into(),
		},
		Element::Text {
			text: format!(
				"Fast expression inserter. Category: `{}` • Total favorites: {}.",
				cat,
				store.favorites.len()
			),
		},
		Element::Button {
			id: "toggle-expand".into(),
			label: format!(
				"Auto Shortcode Expansion (:shrug:): {}",
				if store.auto_expand_shortcodes { "Enabled" } else { "Disabled" }
			),
		},
		Element::Separator,
		Element::Heading {
			text: "Filter Categories".into(),
		},
		Element::Button {
			id: "filter-all".into(),
			label: "Show All".into(),
		},
		Element::Button {
			id: "filter-joy".into(),
			label: "Joy & Happy".into(),
		},
		Element::Button {
			id: "filter-shrug".into(),
			label: "Shrugs".into(),
		},
		Element::Button {
			id: "filter-flip".into(),
			label: "Tableflips".into(),
		},
		Element::Button {
			id: "filter-cool".into(),
			label: "Cool & Lenny".into(),
		},
		Element::Button {
			id: "filter-love".into(),
			label: "Love & Hugs".into(),
		},
		Element::Button {
			id: "filter-sad".into(),
			label: "Sad & Crying".into(),
		},
		Element::Separator,
		Element::Heading {
			text: "Quick Insert Expressions".into(),
		},
		Element::Button {
			id: "insert-shrug".into(),
			label: "¯\\_(ツ)_/¯ (Shrug)".into(),
		},
		Element::Button {
			id: "insert-flip".into(),
			label: "(╯°□°)╯︵ ┻━┻ (Flip)".into(),
		},
		Element::Button {
			id: "insert-unflip".into(),
			label: "┬─┬ノ( º _ ºノ) (Unflip)".into(),
		},
		Element::Button {
			id: "insert-lenny".into(),
			label: "( ͡° ͜ʖ ͡°) (Lenny)".into(),
		},
		Element::Button {
			id: "insert-dealwithit".into(),
			label: "(⌐■_■) (Deal With It)".into(),
		},
		Element::Button {
			id: "insert-hug".into(),
			label: "(づ｡◕‿‿◕｡)づ (Hug)".into(),
		},
		Element::Button {
			id: "insert-cry".into(),
			label: "(ಥ﹏ಥ) (Cry)".into(),
		},
		Element::Separator,
	];

	// List matching kaomoji
	for &(id, text, item_cat) in BUILTIN_KAOMOJI {
		if cat == "all" || cat == item_cat {
			panel.push(Element::Text {
				text: format!("`{}` — {}", id, text),
			});
		}
	}

	let storage = if storage_changed {
		serde_json::to_string(&store).ok()
	} else {
		None
	};

	Output {
		panel,
		replacement,
		storage,
		..Default::default()
	}
}

export!(handle);

#[cfg(test)]
mod tests {
	use super::*;
	use std::collections::BTreeMap;

	#[test]
	fn default_store_contains_initial_favorites() {
		let store = parse_store(None);
		assert_eq!(store.favorites.len(), 7);
		assert!(store.auto_expand_shortcodes);
		assert_eq!(store.active_category, "all");
	}

	#[test]
	fn shortcode_expansion_replaces_known_codes() {
		let store = parse_store(None);
		let text = "Hello :shrug: and goodbye :tableflip:";
		let expanded = expand_shortcodes(text, &store);
		assert!(expanded.contains("¯\\_(ツ)_/¯"));
		assert!(expanded.contains("(╯°□°)╯︵ ┻━┻"));
	}

	#[test]
	fn slash_command_kaomoji_finds_expression() {
		let mut store = parse_store(None);
		let (res, _) = handle_slash_command("/kaomoji joy", &mut store);
		assert!(res.is_some());
		assert!(res.unwrap().contains("(・∀・)"));
	}

	#[test]
	fn slash_command_fav_and_unfav() {
		let mut store = parse_store(None);
		let (res_fav, changed) = handle_slash_command("/expressions fav test (╯°□°)╯", &mut store);
		assert!(changed);
		assert!(res_fav.unwrap().contains("Pinned favorite"));
		assert!(store.favorites.iter().any(|f| f.name == "test"));

		let (res_unfav, changed_unfav) = handle_slash_command("/expressions unfav test", &mut store);
		assert!(changed_unfav);
		assert!(res_unfav.unwrap().contains("Removed favorite"));
		assert!(!store.favorites.iter().any(|f| f.name == "test"));
	}

	#[test]
	fn panel_toggle_expand_and_filter() {
		let inv = Invocation {
			action: "toggle-expand".into(),
			composer: None,
			storage: None,
			selected_message: None,
			values: BTreeMap::new(),
		};
		let out = handle(inv);
		assert!(out.storage.is_some());
		let updated: ExpressionsStore = serde_json::from_str(&out.storage.unwrap()).unwrap();
		assert!(!updated.auto_expand_shortcodes);
	}
}
