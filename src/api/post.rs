use rocket::serde::{ Serialize, Deserialize };
use serde_json::json;

// A Course
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum Post {
	Course(Course),
	Category(Category),
	User(User)
}

impl Post {
	pub fn id(&self) -> String {
		match self {
			Post::Course(co) => co.id.clone(),
			Post::Category(ca) => ca.id.clone(),
			Post::User(u) => u.id.clone()
		}
	}

	pub fn update_by_query(&self) -> sea_orm::prelude::Json {
		match self {
			Post::Course(co) => json!({
				"script": {
					"source": "ctx._source.name = params.name; ctx._source.description = params.description",
					"lang": "painless",
					"params": {
						"name": co.name,
						"description": co.description
					}
				},
				"query": {
					"match": {
						"id": co.id
					}
				}
			}),
			Post::Category(ca) => json!({
				"script": {
					"source": "ctx._source.name = params.name",
					"lang": "painless",
					"params": {
						"name": ca.name
					}
				},
				"query": {
					"match": {
						"id": ca.id
					}
				}
			}),
			Post::User(u) => json!({
				"script": {
					"source": "ctx._source.name = params.name; ctx._source.username = params.username; ctx._source.email = params.email",
					"lang": "painless",
					"params": {
						"name": u.name,
						"username": u.username,
						"email": u.email
					}
				},
				"query": {
					"match": {
						"id": u.id
					}
				}
			})
		}
	}
	
	pub fn base_index(&self) -> sea_orm::prelude::Json {
		json!({
			"mappings": {
				"properties": {
					"id": {
						"type": "text",
						"analyzer": "keyword"
					},
					"name": {
						"type": "text",
						"analyzer": "edge_ngram_analyzer"
					},
					"description": {
						"type": "text",
						"analyzer": "edge_ngram_analyzer"
					},
					"username": {
						"type": "text",
						"analyzer": "edge_ngram_analyzer"
					},
					"email": {
						"type": "text",
						"analyzer": "edge_ngram_analyzer"
					}
				},
				"_routing": {
					"required": true
				},
				"_source": {
					"excludes": ["title_suggest"]
				}
			},
			"settings": {
				"index.number_of_shards": 3,
				"index.number_of_replicas": 0,
				"analysis": {
					"analyzer": {
						"expand": {
							"filter": ["lowercase"],
							"tokenizer": "standard",
							"type": "custom"
						},
						"edge_ngram_analyzer": {
							"filter": [
								"lowercase"
							],
							"tokenizer": "edge_ngram_tokenizer"
						}
					},
					"tokenizer": {
						"edge_ngram_tokenizer": {
							"type": "edge_ngram",
							"min_gram": 2,
							"max_gram": 10,
							"token_chars": [
								"letter",
								"digit"
							]
						}
					}
				}
			}
		})
	}
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Course {
	pub id: String,
	pub name: String,
	pub description: String,
}

impl From<Course> for Post {
	fn from(c: Course) -> Self {
		Post::Course(c)
	}
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Category {
	pub id: String,
	pub name: String
}

impl From<Category> for Post {
	fn from(c: Category) -> Self {
		Post::Category(c)
	}
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct User {
	pub id: String,
	pub name: String,
	pub username: String,
	pub email: String
}

impl From<User> for Post {
	fn from(u: User) -> Self {
		Post::User(u)
	}
}