use elasticsearch::{ DeleteByQueryParts, Elasticsearch, ExistsParts };
use rocket::{http::Status, serde::json::json };
use crate::elastic::client::create_client;
use rocket::response::status::Custom;

static POSTS_INDEX: &'static str = "posts";

#[delete("/<id>")]
pub async fn delete_post(id: &str) -> Result<Custom<sea_orm::prelude::Json>, Custom<sea_orm::prelude::Json>> {
	let client = match create_client() {
		Ok(client) => client,
		Err(err) => return Err(Custom(Status::NotFound, json!({
			"status": Status::NotFound,
			"message": format!("Error al crear el índice: {}", err)
		}))),
	};

	if !check_if_exists(&client, id).await {
		return Err(Custom(Status::NotFound, json!({
			"status": Status::NotFound,
			"message": "Post not found"
		})));
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
			Err(err) => return Err(Custom(Status::InternalServerError, json!({
				"status": Status::InternalServerError,
				"message": format!("Error al buscar el índice: {}", err)
			}))),
		};


	match response.error_for_status_code() {
		Ok(response) => response,
		Err(err) => return Err(Custom(Status::InternalServerError, json!({
			"status": Status::InternalServerError,
			"message": format!("Error al eliminar el indice: {}", err)
		}))),
	};

	Ok(Custom(Status::NoContent, json!({
		"status": Status::NoContent
	})))
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