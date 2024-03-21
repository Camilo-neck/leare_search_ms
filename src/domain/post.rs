use rocket::serde::{ Serialize, Deserialize };
use serde_json::json;
use uuid::Uuid;

// A Course
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum Post {
	Course(Course),
	Category(Category),
	User(User)
}

impl Post {
	pub fn id(&self) -> Uuid {
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
					"source": "ctx._source.name = params.name; ctx._source.description = params.description ; ctx._source.picture = params.picture",
					"lang": "painless",
					"params": {
						"name": co.name,
						"description": co.description,
						"picture": co.picture
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
					"source": "ctx._source.name = params.name; ctx._source.nickname = params.nickname; ctx._source.lastname = params.lastname; ctx._source.picture = params.picture",
					"lang": "painless",
					"params": {
						"name": u.name,
						"lastname": u.lastname,
						"nickname": u.nickname,
						"picture": u.picture
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
					"nickname": {
						"type": "text",
						"analyzer": "edge_ngram_analyzer"
					},
					"lastname": {
						"type": "text",
						"analyzer": "edge_ngram_analyzer"
					},
					"picture": {
						"type": "text",
						"analyzer": "keyword"
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

	pub fn query(&self, q: &str) -> sea_orm::prelude::Json {
		match self {
			Post::Course(_) => json!({
				"query": {
					"multi_match": {
						"query": q,
						"fields": ["name", "description"]
					}
				}
			}),
			Post::Category(_) => json!({
				"query": {
					"multi_match": {
						"query": q,
						"fields": ["name"]
					}
				}
			}),
			Post::User(_) => json!({
				"query": {
					"multi_match": {
						"query": q,
						"fields": ["name", "lastname", "nickname"]
					}
				}
			})
		}
	}
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Course {
	pub id: Uuid,
	pub name: String,
	pub description: String,
	pub picture: String
}

impl From<Course> for Post {
	fn from(c: Course) -> Self {
		Post::Course(c)
	}
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Category {
	pub id: Uuid,
	pub name: String
}

impl From<Category> for Post {
	fn from(c: Category) -> Self {
		Post::Category(c)
	}
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct User {
	pub id: Uuid,
	pub name: String,
	pub lastname: String,
	pub nickname: String,
	pub picture: String,
}

impl From<User> for Post {
	fn from(u: User) -> Self {
		Post::User(u)
	}
}