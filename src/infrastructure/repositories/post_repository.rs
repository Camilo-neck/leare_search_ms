use elasticsearch::indices::{IndicesCreateParts, IndicesDeleteParts, IndicesExistsParts, IndicesPutSettingsParts};
use elasticsearch::{DeleteByQueryParts, SearchParts, UpdateByQueryParts};
use elasticsearch::{BulkOperation, BulkParts, http::StatusCode, ExistsParts};
use rocket::response::status::Custom;
use rocket::{
    http::Status,
    serde::json::{json, Json},
};
use serde_json::Value;
use uuid::Uuid;

use crate::domain::post_repository::PostResult;
use crate::domain::{
    post::Post,
    post_repository::{PostRepository, PostRepositoryImpl},
};


impl PostRepository for PostRepositoryImpl <'_> {
    async fn index(
        &self,
        posts: &[Post],
    ) -> Result<String, Custom<sea_orm::prelude::Json>> {
        // Implement the logic to index a post
		let client = match &self.client {
			Ok(client) => client,
			Err(e) => return Err(Custom(Status::InternalServerError, json!({"message": e.to_string()})))
		};
		let body: Vec<BulkOperation<_>> = posts
			.iter()
			.map(|p| {
				let id = p.id().to_string();
				BulkOperation::index(p).id(&id).routing(&id).into()
			})
			.collect();

		for post in posts {
			if self.check_if_exists(&post.id()).await {
				println!("Post already exists {}", post.id());
				return Ok("Error: Post already exists".to_string());
			}
		}

		let response = client
			.bulk(BulkParts::Index(self.index))
			.body(body)
			.send()
			.await
			.unwrap();

		let json: Value = match response.json().await {
			Ok(json) => json,
			Err(e) => {
				return Err(Custom(Status::InternalServerError, json!({"message": e.to_string()})));
			}
		};

		if json["errors"].as_bool().unwrap() {
			let failed: Vec<&Value> = json["items"]
				.as_array()
				.unwrap()
				.iter()
				.filter(|v| !v["error"].is_null())
				.collect();

			// Rety failures
			let mut retry: Vec<BulkOperation<_>> = Vec::new();
			for f in failed {
				let id = f["index"]["_id"].as_str().unwrap();
				let post = posts
					.iter()
					.find(|p| p.id().to_string() == id)
					.unwrap();
				retry.push(BulkOperation::index(post).id(id).routing(id).into());
			}

			let response = client
				.bulk(BulkParts::Index(self.index))
				.body(retry)
				.send()
				.await
				.unwrap();

			let json: Value = match response.json().await {
				Ok(json) => json,
				Err(e) => {
					return Err(Custom(Status::InternalServerError, json!({"message": e.to_string()})));
				}
			};

			if json["errors"].as_bool().unwrap() {
				return Err(Custom(Status::InternalServerError, json!({"message": "Failed to index posts"})));
			}
		}
			
        Ok("Post indexed successfully".to_string())
    }

	async fn set_refresh_interval(&self, interval: Value) -> Result<(), Custom<sea_orm::prelude::Json>> {
		let client = match &self.client {
			Ok(client) => client,
			Err(e) => return Err(Custom(Status::InternalServerError, json!({"message": e.to_string()})))
		};
		let response = match client
			.indices()
			.put_settings(IndicesPutSettingsParts::Index(&[self.index]))
			.body(json!({"index": {"refresh_interval": interval}}))
			.send()
			.await {
				Ok(response) => response,
				Err(e) => {
					return Err(Custom(Status::InternalServerError, json!({"message": e.to_string()})));
				}
			};

		if !response.status_code().is_success() {
			println!("Failed to set refresh interval: {}", match response.text().await {
				Ok(text) => text,
				Err(e) => e.to_string()
			});
		}

		Ok(())
	}

	async fn check_if_exists(&self, post_id: &Uuid) -> bool {
		let client = match &self.client {
			Ok(client) => client,
			Err(_) => return false
		};
		let response = match client
			.exists(ExistsParts::IndexId(self.index, &post_id.to_string()))
			.routing(&post_id.to_string())
			.pretty(true)
			.send()
			.await {
				Ok(response) => response,
				Err(e) => {
					println!("Failed to check if post exists: {}", e);
					return false;
				}
			};

		if response.status_code().is_success() {
			return true;
		}
		false

	}

	async fn create_index_if_not_exists(&self, delete: bool, post: &Json<Post>) -> Result<(), Custom<sea_orm::prelude::Json>> {
		let client = match &self.client {
			Ok(client) => client,
			Err(e) => return Err(Custom(Status::InternalServerError, json!({"message": e.to_string()})))
		};
		let exists = client
        .indices()
        .exists(IndicesExistsParts::Index(&[self.index]))
        .send()
        .await.unwrap();

		if exists.status_code().is_success() && delete {
			println!("Deleting index, {}", self.index);
			let delete = match client
				.indices()
				.delete(IndicesDeleteParts::Index(&[self.index]))
				.send()
				.await {
					Ok(response) => response,
					Err(e) => {
						return Err(Custom(Status::InternalServerError, json!({"message": e.to_string()})));
					}
				};
			if !delete.status_code().is_success() {
				println!("Failed to delete index: {}", match delete.text().await {
					Ok(text) => text,
					Err(e) => e.to_string()
				});
			}
		}

		if exists.status_code() == StatusCode::NOT_FOUND || delete {
			println!("Creating index: {}", self.index);
			let response = client
				.indices()
				.create(IndicesCreateParts::Index(self.index))
				.body(post.base_index())
				.send()
				.await
				.unwrap();
			if !response.status_code().is_success() {
				println!("Error while creating index");
			}
		}

		Ok(())
	}

    async fn search(
        &self,
        query: sea_orm::prelude::Json,
    ) -> Result<Custom<sea_orm::prelude::Json>, Custom<sea_orm::prelude::Json>> {
		let client = match &self.client {
			Ok(client) => client,
			Err(e) => return Err(Custom(Status::InternalServerError, json!({"message": e.to_string()})))
		};

		let mut response = match client
			.search(SearchParts::Index(&[self.index]))
			.body(query)
			.pretty(true)
			.send()
			.await {
				Ok(response) => response,
				Err(e) => {
					return Err(Custom(Status::InternalServerError, json!({"message": e.to_string()})));
				}
			};

		response = match response.error_for_status_code() {
			Ok(response) => response,
			Err(e) => {
				return Err(Custom(Status::InternalServerError, json!({"message": e.to_string()})));
			}
		};

		let json: Value = match response.json().await {
			Ok(json) => json,
			Err(e) => {
				return Err(Custom(Status::InternalServerError, json!({"message": e.to_string()})));
			}
		};

		println!("Response: {:#?}", json["hits"]["hits"]);

		let posts: Vec<PostResult> = json!(&json["hits"]["hits"])
			.as_array()
			.unwrap()
			.iter()
			.map(|hit| serde_json::from_value(json!({
				"post": hit.get("_source").unwrap_or(&json!({})),
				"highlight": hit.get("highlight").unwrap_or(&json!({})),
			}).clone()).unwrap())
			.collect();

		Ok(Custom(Status::Ok, json!({
			"status": Status::Ok,
			"message": format!("Found {} posts", posts.len()),
			"posts": posts,
		})))
    }

    async fn update(
        &self,
        post_id: &Uuid,
        post: Json<Post>,
    ) -> Result<Custom<sea_orm::prelude::Json>, Custom<sea_orm::prelude::Json>> {
		  let client = match &self.client {
			Ok(client) => client,
			Err(e) => return Err(Custom(Status::InternalServerError, json!({"message": e.to_string()})))
		};

		let response = match client
			.update_by_query(UpdateByQueryParts::Index(&[self.index]))
			.body(post.update_by_query())
			.pretty(true)
			.send()
			.await {
				Ok(response) => response,
				Err(e) => {
					return Err(Custom(Status::InternalServerError, json!({"message": e.to_string()})));
				}
			};
		
		match response.error_for_status_code() {
			Ok(response) => response,
			Err(e) => {
				return Err(Custom(Status::InternalServerError, json!({"message": e.to_string()})));
			}
		};
		
		Ok(Custom(Status::Ok, json!({
			"status": Status::Ok,
			"message": format!("Post {} updated successfully", post_id)
		})))
    }

    async fn delete(
        &self,
        post_id: &Uuid,
    ) -> Result<Custom<sea_orm::prelude::Json>, Custom<sea_orm::prelude::Json>> {
		  let client =  match &self.client {
			Ok(client) => client,
			Err(e) => return Err(Custom(Status::InternalServerError, json!({"message": e.to_string()})))	
		  };

		  let response = match client
		  	.delete_by_query(DeleteByQueryParts::Index(&[self.index]))
			.body(json!({
				"query": {
					"match": {
						"id": post_id.to_string()
					}
				}
			}))
			.pretty(true)
			.send()
			.await {
				Ok(response) => response,
				Err(e) => {
					return Err(Custom(Status::InternalServerError, json!({"message": e.to_string()})));
				}
			};

			match response.error_for_status_code() {
				Ok(response) => response,
				Err(e) => {
					return Err(Custom(Status::InternalServerError, json!({"message": e.to_string()})));
				}
			};

			Ok(Custom(Status::NoContent, json!({
				"status": Status::NoContent
			})))
    }
}
