use crate::Context;

use bottomify::bottom;
use color_eyre::eyre::Result;

#[poise::command(
	slash_command,
	subcommands("to_fahrenheit", "to_celsius", "to_bottom", "from_bottom")
)]
pub async fn convert(_ctx: Context<'_>) -> Result<()> {
	Ok(())
}

/// Ask teawie to convert °F to °C
#[poise::command(slash_command)]
pub async fn to_celsius(
	ctx: Context<'_>,
	#[description = "What teawie will convert"] degrees_fahrenheit: f32,
) -> Result<()> {
	let temp = (degrees_fahrenheit - 32.0) * (5.0 / 9.0);
	ctx.say(temp.to_string()).await?;
	Ok(())
}

/// Ask teawie to convert °C to °F
#[poise::command(slash_command)]
pub async fn to_fahrenheit(
	ctx: Context<'_>,
	#[description = "What teawie will convert"] degrees_celsius: f32,
) -> Result<()> {
	let temp = (degrees_celsius * (9.0 / 5.0)) + 32.0;
	ctx.say(temp.to_string()).await?;
	Ok(())
}

/// Teawie will translate to bottom 🥺
#[poise::command(slash_command)]
pub async fn to_bottom(
	ctx: Context<'_>,
	#[description = "What teawie will translate into bottom"] message: String,
) -> Result<()> {
	let encoded = bottom::encode_string(&message);
	ctx.say(encoded).await?;
	Ok(())
}

/// Teawie will translate from bottom 🥸
#[poise::command(slash_command)]
pub async fn from_bottom(
	ctx: Context<'_>,
	#[description = "What teawie will translate from bottom"] message: String,
) -> Result<()> {
	let decoded = bottom::decode_string(&message)?;
	ctx.say(decoded).await?;
	Ok(())
}
