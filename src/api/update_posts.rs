use elasticsearch::{ExistsParts, UpdateByQueryParts};
use rocket::{http::Status, serde::json::{ json, Json}};
#[cfg(any(feature = "native-tls", feature = "rustls-tls"))]
use elasticsearch::Elasticsearch;
use crate::api::post::Post;
use crate::elastic::client::create_client;
use rocket::response::status::Custom;

static POSTS_INDEX: &'static str = "posts";

#[put("/<post_id>", data="<post>")]
pub async fn update_post(post_id: &str, post: Json<Post>) -> Result<sea_orm::prelude::Json, Custom<String>> {
	let client = match create_client() {
		Ok(client) => client,
		Err(err) => return Err(Custom(Status::NotFound, format!("Error al crear el índice: {}", err).into()))
	};
	if !check_if_exists(&client, post_id).await {
		return Err(Custom(Status::NotFound, "Post not found".into()));
	}
	
	let response = match client
		.update_by_query(UpdateByQueryParts::Index(&[POSTS_INDEX]))
		.body(post.update_by_query())
		.pretty(true)
		.send()
		.await {
			Ok(response) => response,
			Err(err) => return Err(Custom(Status::InternalServerError, format!("Error al buscar el índice: {}", err).into())),
		};

	match response.error_for_status_code() {
		Ok(response) => response,
		Err(err) => return Err(Custom(Status::InternalServerError, format!("Error al actualizar el indice: {}", err).into())),
	};

	Ok(json!({
		"status": Status::Ok,
		"message": format!("Post {} updated successfully", post.id())
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