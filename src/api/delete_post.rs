use elasticsearch::{ DeleteByQueryParts, Elasticsearch, ExistsParts, SearchParts };
use rocket::{http::Status, serde::json::{json, Value} };
use crate::elastic::client::create_client;
use rocket::response::status::Custom;

static POSTS_INDEX: &'static str = "posts";

#[delete("/<id>")]
pub async fn delete_post(id: &str) -> Result<sea_orm::prelude::Json, Custom<String>> {
	let client = match create_client() {
		Ok(client) => client,
		Err(err) => return Err(Custom(Status::NotFound, format!("Error al crear el índice: {}", err).into())),
	};

	if !check_if_exists(&client, id).await {
		return Err(Custom(Status::NotFound, "Post not found".into()));
	}

	let response = match client
		.delete_by_query(DeleteByQueryParts::Index(&[POSTS_INDEX]))
		.body(json!({
			"query": {
				"match": {
					"id": id
				}
			}
		}))
		.pretty(true)
		.send()
		.await {
			Ok(response) => response,
			Err(err) => return Err(Custom(Status::InternalServerError, format!("Error al buscar el índice: {}", err).into())),
		};


	match response.error_for_status_code() {
		Ok(response) => response,
		Err(err) => return Err(Custom(Status::InternalServerError, format!("Error al buscar el índice: {}", err).into())),
	};

	Ok(json!({
		"status": Status::Ok,
		"message": format!("Post {} deleted successfully", id)
	}))
}

pub async fn check_if_exists(client: &Elasticsearch, id: &str) -> bool {
	let response = match client
	.exists(ExistsParts::IndexId(POSTS_INDEX, id))
	.routing(&id)
	.pretty(true)
	.send()
	.await {
		Ok(response) => response,
		Err(_err) => return false,
	};

	if response.status_code().is_success() {
		return true;
	}
	false
}