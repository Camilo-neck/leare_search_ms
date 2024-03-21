use application::use_cases::{delete_post_use_case::DeletePostUseCase, index_post_use_case::IndexPostUseCase, search_post_use_case::SearchPostUseCase, update_post_use_case::UpdatePostUseCase};
use domain::post_repository::PostRepositoryImpl;
use std::sync::Arc;



#[macro_use]
extern crate rocket;

mod domain;
mod application;
mod infrastructure;
mod interfaces;

pub struct App<'a> {
    pub post_repository: Arc<PostRepositoryImpl<'a>>,
    pub search_post_use_case: SearchPostUseCase<'a>,
    pub index_post_use_case: IndexPostUseCase<'a>,
    pub update_post_use_case: UpdatePostUseCase<'a>,
    pub delete_post_use_case: DeletePostUseCase<'a>,
}

impl <'a> App <'a> {
    pub fn new() -> Self {
        let post_repository = Arc::new(PostRepositoryImpl::new());
        App {
            post_repository: post_repository.clone(),
            search_post_use_case: SearchPostUseCase::new(post_repository.clone()),
            index_post_use_case: IndexPostUseCase::new(post_repository.clone()),
            update_post_use_case: UpdatePostUseCase::new(post_repository.clone()),
            delete_post_use_case: DeletePostUseCase::new(post_repository.clone()),
        }
    }

}


#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let app = App::new();
    rocket::build()
    .manage(app)
    .mount("/posts", interfaces::routes::routes())
    .launch()
    .await?;

    Ok(())
}