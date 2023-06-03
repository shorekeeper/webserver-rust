// not works too
use actix_multipart::Multipart;
use actix_web::{web, Error, HttpResponse};
use futures::{StreamExt, TryStreamExt};
use serde::Deserialize;
use std::io::Write;

#[derive(Deserialize)]
struct Query {
    id: i32,
}

async fn save_file(mut payload: Multipart) -> Result<HttpResponse, Error> {
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field.content_disposition().unwrap();
        let filename = content_type.get_filename().unwrap();
        let filepath = format!("./tmp/{}", sanitize_filename::sanitize(&filename));

        let mut f = web::block(|| std::fs::File::create(filepath))
            .await
            .unwrap();

        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            f = web::block(move || f.write_all(&data).map(|_| f)).await?;
        }
    }
    Ok(HttpResponse::Ok().into())
}

async fn get_data(query: web::Query<Query>) -> HttpResponse {
    // Retrieve data from database using the provided id
    let data = get_data_from_database(query.id).await;

    // Return data as a JSON response
    HttpResponse::Ok().json(data)
}

async fn get_data_from_database(id: i32) -> serde_json::Value {
    // Example data retrieval from database
    json!({
        "id": id,
        "name": "Example Data",
        "value": 42
    })
}