use std::sync::Arc;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::{Json, json};

use crate::domain::post::Post;
use crate::domain::post_repository::{PostRepository, PostRepositoryImpl};

pub struct IndexPostUseCase<'a> {
	post_repository: Arc<PostRepositoryImpl<'a>>,
}

impl <'a> IndexPostUseCase <'a> {
	pub fn new(post_repository: Arc<PostRepositoryImpl<'a>>) -> Self {
		IndexPostUseCase { post_repository }
	}

	pub async fn execute(&self, post: Json<Post>) -> Result<Custom<sea_orm::prelude::Json>, Custom<sea_orm::prelude::Json>> {
		if let Err(err) = self.post_repository.create_index_if_not_exists(false, &post).await {
			return Err(err);
		};

		if let Err(err)  = self.post_repository.set_refresh_interval(json!("-1")).await {
			return Err(err)
		};
		
		match self.post_repository.index(&[post.into_inner()]).await {
			Ok(response) => {
				if response.contains("Error") {
					return Err(Custom(Status::InternalServerError, json!({
						"success": false,
						"message": response
					})))
				}
			},
			Err(e) => return Err(e)
		};

		if let Err(err)  = self.post_repository.set_refresh_interval(json!(null)).await {
			return Err(err)
		};

		Ok(Custom(Status::Created, json!({
			"success": true,
			"message": "Post indexed successfully"
		})))
	}
}