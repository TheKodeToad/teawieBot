use crate::api::REQWEST_CLIENT;

use color_eyre::eyre::{eyre, Result};
use log::*;
use reqwest::StatusCode;
use serde::Deserialize;

const SHIGGY: &str = "https://safebooru.donmai.us";
const RANDOM_SHIGGY: &str = "/posts/random.json?tags=kemomimi-chan_(naga_u)+naga_u&only=file_url";

#[derive(Deserialize)]
struct SafebooruResponse {
	file_url: String,
}

pub async fn get_random_shiggy() -> Result<String> {
	let req = REQWEST_CLIENT
		.get(format!("{SHIGGY}{RANDOM_SHIGGY}"))
		.build()?;

	info!("making request to {}", req.url());
	let resp = REQWEST_CLIENT.execute(req).await?;
	let status = resp.status();

	if let StatusCode::OK = status {
		let data = resp.json::<SafebooruResponse>().await?;
		Ok(data.file_url)
	} else {
		error!(
			"couldn't fetch random teawie from {}! {}",
			resp.url(),
			status
		);

		Err(eyre!("failed to get random teawie with {status}"))
	}
}
