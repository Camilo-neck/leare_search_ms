use std::sync::Arc;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::json;
use uuid::Uuid;

use crate::domain::post_repository::{PostRepository, PostRepositoryImpl};

pub struct DeletePostUseCase<'a> {
	post_repository: Arc<PostRepositoryImpl<'a>>,
}

impl <'a> DeletePostUseCase <'a> {
	pub fn new(post_repository: Arc<PostRepositoryImpl<'a>>) -> Self {
		DeletePostUseCase { post_repository }
	}

	pub async fn execute(&self, post_id: &Uuid) -> Result<Custom<sea_orm::prelude::Json>, Custom<sea_orm::prelude::Json>> {
		if !self.post_repository.check_if_exists(post_id).await {
			return Err(Custom(Status::NotFound, json!({
				"status": Status::NotFound,
				"message": "Post not found"
			})))
		}

		match self.post_repository.delete(post_id).await {
			Ok(response) => return Ok(response),
			Err(e) => return Err(e)
		};
	}
}