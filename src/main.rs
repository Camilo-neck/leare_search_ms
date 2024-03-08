#![forbid(unsafe_code)]
// Warn on generally recommended lints that are not enabled by default.
#![warn(future_incompatible, rust_2018_idioms, unused, macro_use_extern_crate)]
// Warn when we write more code than necessary.
#![warn(unused_lifetimes, single_use_lifetimes, unreachable_pub, trivial_casts)]
// Warn when we don't implement (derive) commonly needed traits. May be too strict.
#![warn(missing_copy_implementations, missing_debug_implementations)]
// Turn on some extra Clippy (Rust code linter) warnings. Run `cargo clippy`.
#![warn(clippy::all)]

#[macro_use] extern crate rocket;
use rocket::serde::{Deserialize, Serialize, json::{Json, json}};
use rocket::http::Status;
use api::{delete_post::delete_post, index_post::*, search_post::search_post, update_posts::update_post};

pub mod api;
pub mod elastic;


#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct Post<'r> {
    id: i32,
    name: &'r str,
    description: &'r str,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Res {
    status: Status,
    message: String,
    id: i32,
    name: String,
    description: String,
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/create", data = "<post>")]
fn create(post: Json<Post<'_>>) -> Result<sea_orm::prelude::Json, Status> {
    let mut response = rocket::response::Response::new();
    response.set_header(rocket::http::Header::new("Custom-Header", "Custom Value"));
    Ok(json!({
        "status": Status::Created,
        "message": format!("Post created with id: {}", post.id),
        "id": post.id,
        "name": String::from(post.name),
        "description": String::from(post.description),
    }))
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/", routes![index, create])
        .mount("/courses", routes![index_post, search_post, update_post, delete_post])
        .launch()
        .await?;

    Ok(())
}