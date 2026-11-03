use serein_extension_sdk::{export, Element, Invocation, Output};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct UserReview {
	pub id: String,
	pub target_user: String,
	pub reviewer_name: String,
	pub rating: i8, // 1 to 5
	pub comment: String,
	pub timestamp: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReviewStore {
	pub reviews: Vec<UserReview>,
	pub notify_on_review: bool,
	pub min_rating_filter: i8,
}

impl Default for ReviewStore {
	fn default() -> Self {
		Self {
			reviews: vec![
				UserReview {
					id: "rev-101".into(),
					target_user: "VenSerein".into(),
					reviewer_name: "CommunityMember".into(),
					rating: 5,
					comment: "Top notch performance and native desktop UX!".into(),
					timestamp: "Today".into(),
				},
				UserReview {
					id: "rev-102".into(),
					target_user: "Luna".into(),
					reviewer_name: "DevPeer".into(),
					rating: 5,
					comment: "Great open source contributor and reliable developer.".into(),
					timestamp: "Yesterday".into(),
				},
				UserReview {
					id: "rev-103".into(),
					target_user: "Alice".into(),
					reviewer_name: "Bob".into(),
					rating: 4,
					comment: "Friendly trading partner, smooth transactions.".into(),
					timestamp: "2 days ago".into(),
				},
			],
			notify_on_review: true,
			min_rating_filter: 1,
		}
	}
}

pub fn parse_store(storage: Option<&str>) -> ReviewStore {
	storage
		.and_then(|s| serde_json::from_str::<ReviewStore>(s).ok())
		.unwrap_or_default()
}

pub fn format_user_reviews(target: &str, store: &ReviewStore) -> String {
	let matches: Vec<&UserReview> = store
		.reviews
		.iter()
		.filter(|r| r.target_user.eq_ignore_ascii_case(target))
		.collect();

	if matches.is_empty() {
		return format!("⭐ **ReviewDB:** No reviews found for `{}` yet.", target);
	}

	let total_stars: i64 = matches.iter().map(|r| r.rating as i64).sum();
	let avg: f64 = total_stars as f64 / matches.len() as f64;
	let mut out = format!(
		"⭐ **ReviewDB Profile: {}**\nAverage Rating: **{:.1}/5** ({} review{})\n\n",
		target,
		avg,
		matches.len(),
		if matches.len() == 1 { "" } else { "s" }
	);

	for r in matches {
		let stars = "★".repeat(r.rating.clamp(1, 5) as usize);
		out.push_str(&format!(
			"• `{}` by **{}** ({})\n  *\"{}\"*\n",
			stars, r.reviewer_name, r.timestamp, r.comment
		));
	}
	out
}

pub fn handle_slash_command(cmd: &str, store: &mut ReviewStore) -> (Option<String>, bool) {
	let parts: Vec<&str> = cmd.split_whitespace().collect();
	if parts.is_empty() {
		return (None, false);
	}

	let root = parts[0];
	if root != "/review" && root != "/reviewdb" {
		return (None, false);
	}

	if parts.len() == 1 || parts[1] == "help" {
		let help = "🛡️ **ReviewDB Reputation Commands**\n\
		• `/review <user>` — View user rating and reviews\n\
		• `/review add <user> <1-5> <comment>` — Submit reputation review\n\
		• `/review list` — List all cached reviews\n\
		• `/review delete <id>` — Remove review by id";
		return (Some(help.into()), false);
	}

	let sub = parts[1];
	if sub == "list" {
		let mut out = String::from("📑 **Cached ReviewDB Database**\n\n");
		for r in &store.reviews {
			let stars = "★".repeat(r.rating.clamp(1, 5) as usize);
			out.push_str(&format!(
				"• `[{}]` **{}** -> **{}** (`{}`): *{}*\n",
				r.id, r.reviewer_name, r.target_user, stars, r.comment
			));
		}
		return (Some(out), false);
	}

	if sub == "add" {
		if parts.len() >= 5 {
			let target = parts[2].trim().trim_start_matches('@');
			let rating = parts[3].parse::<i8>().unwrap_or(5).clamp(1, 5);
			let comment = parts[4..].join(" ");
			let id = format!("rev-{}", store.reviews.len() + 100);
			if store.reviews.len() >= 100 {
				store.reviews.remove(0);
			}
			store.reviews.push(UserReview {
				id: id.clone(),
				target_user: target.to_owned(),
				reviewer_name: "You".into(),
				rating,
				comment: comment.clone(),
				timestamp: "Just now".into(),
			});
			return (
				Some(format!(
					"✅ Submitted review `[{}]` for **{}** ({}★): \"{}\"",
					id, target, rating, comment
				)),
				true,
			);
		} else {
			return (
				Some("Usage: `/review add <user> <rating 1-5> <comment>`".into()),
				false,
			);
		}
	}

	if sub == "delete" {
		if parts.len() >= 3 {
			let id = parts[2];
			let before = store.reviews.len();
			store.reviews.retain(|r| r.id != id);
			let removed = store.reviews.len() < before;
			if removed {
				return (Some(format!("🗑️ Deleted review `[{}]`", id)), true);
			} else {
				return (Some(format!("Review `[{}]` not found.", id)), false);
			}
		} else {
			return (Some("Usage: `/review delete <id>`".into()), false);
		}
	}

	// Default: look up user reviews
	let target = sub.trim().trim_start_matches('@');
	(Some(format_user_reviews(target, store)), false)
}

pub fn handle(input: Invocation) -> Output {
	let mut store = parse_store(input.storage.as_deref());
	let action = input.action.as_str();
	let mut storage_changed = false;

	// Slash command expansion
	if action == "expand-command" || action == "expand-reviews" {
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
							text: "ReviewDB Hub".into(),
						},
						Element::Text {
							text: "ReviewDB command executed.".into(),
						},
					],
					storage,
					..Default::default()
				};
			}
		}
	}

	if action == "toggle-notify" {
		store.notify_on_review = !store.notify_on_review;
		storage_changed = true;
	} else if action == "clear-all" {
		store.reviews.clear();
		storage_changed = true;
	}

	let replacement = match action {
		"export-summary" => {
			let mut out = String::from("⭐ **ReviewDB Reputation Summary**\n\n");
			for r in &store.reviews {
				out.push_str(&format!(
					"• **{}**: {}★ by {} — *\"{}\"*\n",
					r.target_user, r.rating, r.reviewer_name, r.comment
				));
			}
			Some(out)
		}
		_ => None,
	};

	let mut panel = vec![
		Element::Heading {
			text: "ReviewDB — User Reputation & Ratings Hub".into(),
		},
		Element::Text {
			text: format!(
				"Cached reviews: {} total. Notifications: {}.",
				store.reviews.len(),
				if store.notify_on_review {
					"Enabled"
				} else {
					"Disabled"
				}
			),
		},
		Element::Button {
			id: "toggle-notify".into(),
			label: format!(
				"Notify on New Reviews: {}",
				if store.notify_on_review {
					"Enabled"
				} else {
					"Disabled"
				}
			),
		},
		Element::Button {
			id: "export-summary".into(),
			label: "Export Reputation Summary to Composer".into(),
		},
		Element::Button {
			id: "clear-all".into(),
			label: "Clear Cached Reviews".into(),
		},
		Element::Separator,
		Element::Heading {
			text: "Recent Community Reviews".into(),
		},
	];

	for r in &store.reviews {
		let stars = "★".repeat(r.rating.clamp(1, 5) as usize);
		panel.push(Element::Heading {
			text: format!("{} ({})", r.target_user, stars),
		});
		panel.push(Element::Text {
			text: format!(
				"Reviewed by {} ({})\n\"{}\"",
				r.reviewer_name, r.timestamp, r.comment
			),
		});
		panel.push(Element::Separator);
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
	fn default_store_contains_initial_reviews() {
		let store = parse_store(None);
		assert_eq!(store.reviews.len(), 3);
		assert!(store.notify_on_review);
	}

	#[test]
	fn slash_command_user_lookup() {
		let mut store = parse_store(None);
		let (res, _) = handle_slash_command("/review Luna", &mut store);
		assert!(res.is_some());
		assert!(res.unwrap().contains("ReviewDB Profile: Luna"));
	}

	#[test]
	fn slash_command_add_and_delete() {
		let mut store = parse_store(None);
		let (res_add, changed) =
			handle_slash_command("/review add Tester 5 Awesome developer!", &mut store);
		assert!(changed);
		assert!(res_add.unwrap().contains("Submitted review"));
		assert!(store.reviews.iter().any(|r| r.target_user == "Tester"));

		let added_id = store.reviews.last().unwrap().id.clone();
		let (res_del, changed_del) =
			handle_slash_command(&format!("/review delete {}", added_id), &mut store);
		assert!(changed_del);
		assert!(res_del.unwrap().contains("Deleted review"));
		assert!(!store.reviews.iter().any(|r| r.id == added_id));
	}

	#[test]
	fn panel_actions_toggle_notify() {
		let inv = Invocation {
			action: "toggle-notify".into(),
			composer: None,
			storage: None,
			selected_message: None,
			values: BTreeMap::new(),
		};
		let out = handle(inv);
		assert!(out.storage.is_some());
		let updated: ReviewStore = serde_json::from_str(&out.storage.unwrap()).unwrap();
		assert!(!updated.notify_on_review);
	}
}
