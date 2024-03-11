#[macro_use] extern crate rocket;
use api::{delete_post::delete_post, index_post::*, search_post::search_post, update_posts::update_post};

pub mod api;
pub mod elastic;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/", routes![index])
        .mount("/posts", routes![index_post, search_post, update_post, delete_post])
        .launch()
        .await?;

    Ok(())
}