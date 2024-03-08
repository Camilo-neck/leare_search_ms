use elasticsearch::SearchParts;
use rocket::{http::Status, serde::json::{json, Value}};
use crate::api::post::Post;
use crate::elastic::client::create_client;
use rocket::response::status::Custom;

static POSTS_INDEX: &'static str = "posts";

#[get("/?<q>")]
pub async fn search_post(q: &str) -> Result<sea_orm::prelude::Json, Custom<String>> {
	let query = if q.is_empty() {
		json!({
			"query": {
				"match_all": {}
			}
		})
	} else {
		json!({
			"query": {
				"multi_match": {
					"query": q,
					"fields": ["name", "description"],
					"operator": "and",
					"type": "cross_fields",
				}
			}
		})
	};
	let client = match create_client() {
		Ok(client) => client,
		Err(err) => return Err(Custom(Status::NotFound, format!("Error al crear el índice: {}", err).into())),
	};
	let mut response = match client
		.search(SearchParts::Index(&[POSTS_INDEX]))
		.body(query)
		.pretty(true)
		.send()
		.await {
			Ok(response) => response,
			Err(err) => return Err(Custom(Status::InternalServerError, format!("Error al buscar el índice: {}", err).into())),
		};

	response = match response.error_for_status_code() {
		Ok(response) => response,
		Err(err) => return Err(Custom(Status::InternalServerError, format!("Error al buscar el índice: {}", err).into())),
	};

	let json: Value = match response.json().await {
		Ok(value) => value,
		Err(err) => return Err(Custom(Status::InternalServerError, format!("Error parsing JSON response: {}", err).into())),
	};
	let posts: Vec<Post> = json!(&json["hits"]["hits"])
		.as_array()
		.unwrap()
		.iter()
		.map(|hit| serde_json::from_value(hit["_source"].clone()).unwrap())
		.collect();
	println!("Searching for: {}", q);
    Ok(json!({
		"status": Status::Ok,
		"message": format!("Found {} posts", posts.len()),
		"posts": posts,
	}))
}