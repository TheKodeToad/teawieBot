use crate::{consts, utils};
use log::*;
use poise::serenity_prelude::{ChannelId, EmojiId, GuildId, MessageReaction, ReactionType};

#[derive(Clone)]
pub struct Settings {
	pub allowed_guilds: Vec<GuildId>,
	pub pinboard_target: ChannelId,
	pub pinboard_sources: Option<Vec<ChannelId>>,
	pub reactboard_target: ChannelId,
	pub reactboard_requirement: Option<u64>,
	pub reactboard_custom_reactions: Vec<EmojiId>,
	pub reactboard_unicode_reactions: Vec<String>,
}

impl Settings {
	pub fn new() -> Option<Self> {
		let allowed_guilds = utils::parse_snowflakes_from_env("ALLOWED_GUILDS", GuildId)
			.unwrap_or_else(|| vec![consts::TEAWIE_GUILD, GuildId(1091969030694375444)]);

		let Some(pinboard_target) = utils::parse_snowflake_from_env("PIN_BOARD_TARGET", ChannelId)
		else {
			return None;
		};
		let pinboard_sources = utils::parse_snowflakes_from_env("PIN_BOARD_SOURCES", ChannelId);

		let Some(reactboard_target) =
			utils::parse_snowflake_from_env("REACT_BOARD_TARGET", ChannelId)
		else {
			return None;
		};

		let reactboard_requirement = utils::parse_snowflake_from_env("REACT_BOARD_MIN", u64::from);

		let reactboard_custom_reactions =
			utils::parse_snowflakes_from_env("REACT_BOARD_CUSTOM_REACTIONS", EmojiId)
				.unwrap_or_default();

		let reactboard_unicode_reactions = std::env::var("REACT_BOARD_UNICODE_REACTIONS")
			.ok()
			.map(|v| {
				v.split(',')
					.map(|vs| vs.to_string())
					.collect::<Vec<String>>()
			})
			.unwrap_or_default();

		info!("PinBoard target is {}", pinboard_target);
		if let Some(sources) = &pinboard_sources {
			info!("PinBoard sources are {:#?}", sources);
		}
		info!("ReactBoard target is {}", reactboard_target);
		info!(
			"ReactBoard custom reactions are {:#?}",
			reactboard_custom_reactions
		);
		info!(
			"ReactBoard unicode reactions are {:#?}",
			reactboard_unicode_reactions
		);

		Some(Self {
			allowed_guilds,
			pinboard_target,
			pinboard_sources,
			reactboard_target,
			reactboard_requirement,
			reactboard_custom_reactions,
			reactboard_unicode_reactions,
		})
	}

	pub fn can_use_reaction(&self, reaction: &MessageReaction) -> bool {
		match &reaction.reaction_type {
			ReactionType::Custom {
				animated: _,
				id,
				name: _,
			} => self.reactboard_custom_reactions.contains(id),
			ReactionType::Unicode(name) => self.reactboard_unicode_reactions.contains(name),
			// no other types exist yet, so assume we can't use them :p
			_ => false,
		}
	}

	pub fn is_guild_allowed(&self, gid: GuildId) -> bool {
		self.allowed_guilds.contains(&gid)
	}
}
