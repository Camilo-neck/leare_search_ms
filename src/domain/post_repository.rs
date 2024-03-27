use elasticsearch::Elasticsearch;
use rocket::{response::status::Custom, serde::json::Json};
use serde_json::Value;
use uuid::Uuid;

use crate::infrastructure::client::create_client;

use super::post::Post;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct PostResult {
	pub post: Post,
	pub highlight: Value,
}

pub trait PostRepository {
	async fn search(&self, query: sea_orm::prelude::Json) -> Result<Custom<sea_orm::prelude::Json>, Custom<sea_orm::prelude::Json>>;
	async fn set_refresh_interval(&self, interval: sea_orm::prelude::Json) -> Result<(), Custom<sea_orm::prelude::Json>>;
	async fn check_if_exists(&self, post_id: &Uuid) -> bool;
	async fn create_index_if_not_exists(&self, delete: bool, post: &Json<Post>) -> Result<(), Custom<sea_orm::prelude::Json>>;
	async fn index(&self, posts: &[Post]) ->  Result<String, Custom<sea_orm::prelude::Json>>;
	async fn update(&self, post_id: &Uuid, post: Json<Post>) ->  Result<Custom<sea_orm::prelude::Json>, Custom<sea_orm::prelude::Json>>;
	async fn delete(&self, post_id: &Uuid) ->  Result<Custom<sea_orm::prelude::Json>, Custom<sea_orm::prelude::Json>>;
}

pub struct PostRepositoryImpl<'a> {
	pub client: Result<Elasticsearch, elasticsearch::Error>,
	pub index: &'a str 
}

impl <'a> PostRepositoryImpl <'a> {
	pub fn new() -> Self {
		PostRepositoryImpl {
			client: create_client(),
			index: "posts"
		}
	}
}