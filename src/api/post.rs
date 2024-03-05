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
	pub fn id(&self) -> i32 {
		match self {
			Post::Course(co) => co.id,
			Post::Category(ca) => ca.id,
			Post::User(u) => u.id
		}
	}
	
	pub fn base_index(&self) -> sea_orm::prelude::Json {
		match self {
			Post::Course(_) => json!({
                    "mappings": {
                        "properties": {
                            "id": {
                                "type": "integer"
                            },
                            "name": {
                                "type": "text",
                                "analyzer": "expand"
                            },
                            "description": {
                                "type": "text",
                                "analyzer": "expand"
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
                                "filter": ["lowercase", "stop"],
                                "tokenizer": "standard",
                                "type": "custom"
                                }
                            },
                        }
                    }
                }),
			Post::Category(_) => json!({
                    "mappings": {
                        "properties": {
                            "id": {
                                "type": "integer"
                            },
                            "name": {
                                "type": "text",
                                "analyzer": "expand"
                            },
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
                                }
                            },
                        }
                    }
                }),
			Post::User(_) => json!({
					"mappings": {
						"properties": {
							"id": {
								"type": "integer"
							},
							"name": {
								"type": "text",
								"analyzer": "expand"
							},
							"username": {
								"type": "text",
								"analyzer": "expand"
							},
							"email": {
								"type": "text",
								"analyzer": "expand"
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
								}
							},
						}
					}
				})
		}
	}
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Course {
	pub id: i32,
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
	pub id: i32,
	pub name: String
}

impl From<Category> for Post {
	fn from(c: Category) -> Self {
		Post::Category(c)
	}
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct User {
	pub id: i32,
	pub name: String,
	pub username: String,
	pub email: String
}

impl From<User> for Post {
	fn from(u: User) -> Self {
		Post::User(u)
	}
}