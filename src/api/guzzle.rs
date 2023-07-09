use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct GuzzleResponse {
	pub url: String,
}

const GUZZLE: &str = "https://api.mydadleft.me";

pub async fn get_random_teawie() -> String {
	let endpoint = "get_random_teawie";
	let resp = reqwest::get(format!("{GUZZLE}/{endpoint}")).await.unwrap(); // why did i have to own
																		// this constant? i have
																		// no idea!
	let err_msg = "couldn't get a teawie";

	match resp.status() {
		StatusCode::OK => match resp.json::<GuzzleResponse>().await {
			Ok(data) => data.url,
			Err(why) => format!("{} ({:?})", err_msg, why),
		},
		other => format!("{} ({:?})", err_msg, other),
	}
}
