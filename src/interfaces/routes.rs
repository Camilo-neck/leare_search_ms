use rocket::{response::status::Custom, serde::json::Json, Route, State};
use uuid::Uuid;

use crate::{domain::post::Post, App};

pub fn routes() -> Vec<Route> {
    // Define your routes here
    routes![
        get_users,
        create_user,
        get_user,
        delete_user,
        search_post,
        index_post,
        update_post,
        delete_post
    ]
}

#[get("/users")]
fn get_users() -> &'static str {
    // Implement the logic to get all users
    "Get all users"
}

#[post("/users")]
fn create_user() -> &'static str {
    // Implement the logic to create a user
    "Create a user"
}

#[get("/users/<id>")]
fn get_user(id: u32) -> String {
    // Implement the logic to get a user by ID
    format!("Get user with ID: {}", id)
}

#[delete("/users/<id>")]
fn delete_user(id: u32) -> String {
    // Implement the logic to delete a user by ID
    format!("Delete user with ID: {}", id)
}

#[get("/?<q>")]
async fn search_post(q: &str, app: &State<App<'_>>) -> Result<Custom<sea_orm::prelude::Json>, Custom<sea_orm::prelude::Json>> {
    // Implement the logic to search a post
    app.search_post_use_case.execute(q).await
}

#[post("/", data="<post>")]
async fn index_post(post: Json<Post>, app: &State<App<'_>>) -> Result<Custom<sea_orm::prelude::Json>, Custom<sea_orm::prelude::Json>> {
    app.index_post_use_case.execute(post).await
}

#[put("/<post_id>", data="<post>")]
async fn update_post(post_id: &str, post: Json<Post>, app: &State<App<'_>>) -> Result<Custom<sea_orm::prelude::Json>, Custom<sea_orm::prelude::Json>> {
    // Implement the logic to update a post
    let id = Uuid::parse_str(post_id).unwrap();
    app.update_post_use_case.execute(&id, post).await
}

#[delete("/<post_id>")]
async fn delete_post(post_id: &str, app: &State<App<'_>>) -> Result<Custom<sea_orm::prelude::Json>, Custom<sea_orm::prelude::Json>> {
    // Implement the logic to delete a post
    let id = Uuid::parse_str(post_id).unwrap();
    app.delete_post_use_case.execute(&id).await
}