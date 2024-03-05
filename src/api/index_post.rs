use rocket::{http::Status, serde::json::{ json, Json}};
#[cfg(any(feature = "native-tls", feature = "rustls-tls"))]
use elasticsearch::{
    http::StatusCode,
    indices::{
        IndicesCreateParts, IndicesDeleteParts, IndicesExistsParts, IndicesPutSettingsParts,
    },
    BulkOperation, BulkParts, Elasticsearch, Error,
};
use serde_json::Value;
use crate::api::post::Post;
use crate::elastic::client::create_client;
use rocket::response::status::Custom;

static COURSES_INDEX: &'static str = "posts";

#[post("/", data="<courses>")]
pub async fn index_post(courses: Json<Post>) -> Result<(), Custom<String>> {
	let client = match create_client() {
        Ok(client) => client,
        Err(err) => return Err(Custom(Status::NotFound, format!("Error al crear el índice: {}", err).into()))
    };
    if let Err(err) = create_index_if_not_exists(&client, false, &courses).await {
        return Err(Custom(Status::NoContent, format!("Error al crear el índice: {}", err).into()));
    };
    if let Err(err) = set_refresh_interval(&client, json!("-1")).await {
        return Err(Custom(Status::NotAcceptable, format!("Error al crear el índice: {}", err).into()));
    };

    if let Err(err) = index_posts(&client, &[courses.into_inner()]).await {
        return Err(Custom(Status::NotImplemented, format!("Error al crear el índice: {}", err).into()));
    };

    if let Err(err) = set_refresh_interval(&client, json!(null)).await {
        return Err(Custom(Status::NotExtended, format!("Error al crear el índice: {}", err).into()));
    };
    Ok(())
}

async fn set_refresh_interval(client: &Elasticsearch, interval: Value) -> Result<(), Error> {
    let response = client
        .indices()
        .put_settings(IndicesPutSettingsParts::Index(&[COURSES_INDEX]))
        .body(json!({
            "index" : {
                "refresh_interval" : interval
            }
        }))
        .send()
        .await?;

    if !response.status_code().is_success() {
        println!("Failed to update refresh interval");
    }

    Ok(())
}

async fn index_posts(client: &Elasticsearch, courses: &[Post]) -> Result<(), Error>  {
    let body: Vec<BulkOperation<_>> = courses
    .iter()
    .map(|c| {
        let id = c.id().to_string();
        BulkOperation::index(c).id(&id).routing(&id).into()
    })
    .collect();

    let response = client
    .bulk(BulkParts::Index(COURSES_INDEX))
    .body(body)
    .send()
    .await?;

    let json: Value = response.json().await?;

    if json["errors"].as_bool().unwrap() {
        let failed: Vec<&Value> = json["items"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|v| !v["error"].is_null())
            .collect();

        // TODO: retry failures
        println!("Errors whilst indexing. Failures: {}", failed.len());
    }
    Ok(())
}

async fn create_index_if_not_exists(client: &Elasticsearch, delete: bool, post: &Json<Post>) -> Result<(), Error> {
	let exists = client
        .indices()
        .exists(IndicesExistsParts::Index(&[COURSES_INDEX]))
        .send()
        .await?;

	if exists.status_code().is_success() && delete {
        println!("Deleting existing index: {}", COURSES_INDEX);
        let delete = client
            .indices()
            .delete(IndicesDeleteParts::Index(&[COURSES_INDEX]))
            .send()
            .await?;

        if !delete.status_code().is_success() {
            println!("Problem deleting index: {}", delete.text().await?);
        }
    }

	if exists.status_code() == StatusCode::NOT_FOUND || delete {
        println!("Creating index: {}", COURSES_INDEX);
		let response = client
			.indices()
			.create(IndicesCreateParts::Index(COURSES_INDEX))
            .body(post.base_index())
            .send()
            .await?;
        if !response.status_code().is_success() {
            println!("Error while creating index");
        }
	}
    Ok(())
}