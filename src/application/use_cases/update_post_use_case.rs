use std::sync::Arc;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::{Json, json};
use uuid::Uuid;

use crate::domain::post::Post;
use crate::domain::post_repository::{PostRepository, PostRepositoryImpl};

pub struct UpdatePostUseCase<'a> {
	post_repository: Arc<PostRepositoryImpl<'a>>,
}

impl <'a> UpdatePostUseCase <'a> {
	pub fn new(post_repository: Arc<PostRepositoryImpl<'a>>) -> Self {
		UpdatePostUseCase { post_repository }
	}

	pub async fn execute(&self, post_id: &Uuid, post: Json<Post>) -> Result<Custom<sea_orm::prelude::Json>, Custom<sea_orm::prelude::Json>> {
		if !self.post_repository.check_if_exists(post_id).await {
			return Err(Custom(Status::NotFound, json!({
				"status": Status::NotFound,
				"message": "Post not found"
			})))
		}

		match self.post_repository.update(post_id, post).await {
			Ok(response) => return Ok(response),
			Err(e) => return Err(e)
		};
	}
}