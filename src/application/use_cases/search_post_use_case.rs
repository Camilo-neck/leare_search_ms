use std::sync::Arc;
use rocket::response::status::Custom;
use rocket::serde::json::json;

use crate::domain::post_repository::{PostRepository, PostRepositoryImpl};

pub struct SearchPostUseCase<'a> {
	post_repository: Arc<PostRepositoryImpl<'a>>,
}

impl <'a> SearchPostUseCase <'a> {
	pub fn new(post_repository: Arc<PostRepositoryImpl<'a>>) -> Self {
		SearchPostUseCase { post_repository }
	}

	pub async fn execute(&self, q: &str) -> Result<Custom<sea_orm::prelude::Json>, Custom<sea_orm::prelude::Json>> {
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
						"fields": ["name", "lastname", "nickname", "description"],
						"operator": "and",
						"type": "cross_fields",
					}
				},
				"highlight": {
					"fields": {
						"name": {},
						"lastname": {},
						"nickname": {},
						"description": {}
					},
					"pre_tags": ["<b>"],
					"post_tags": ["</b>"]
				}
			})
		};
        println!("Cluster address: {:?}", std::env::var("ELASTICSEARCH_URL").unwrap());

		match self.post_repository.search(query).await {
			Ok(response) => return Ok(response),
			Err(e) => return Err(e)
		};
	}
}