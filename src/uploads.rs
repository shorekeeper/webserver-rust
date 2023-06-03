
// not works 👍
use actix_multipart::{Field, Multipart};
use actix_web::{web, App, Error, HttpResponse};
use futures::{StreamExt, TryStreamExt};
use actix_web::dev::{ServiceRequest, ServiceResponse, ServiceFactory};
async fn save_field(mut field: Field) -> Result<(), Error> {
    // get the content type and filename from the field's content disposition
    let content_type = field.content_disposition().unwrap();
    let filename = content_type.get_filename().unwrap();
    
    // create a new file with the given filename
    let mut file = web::block(|| std::fs::File::create(filepath))
        .await
        .unwrap();

    while let Some(chunk) = field.next().await {
        let data = chunk.unwrap();
        file = web::block(move || {
            file.write_all(&data).map_err(actix_web::error::BlockingError::Error)?;
            Ok(file)
        })
        .await?;
    }
    Ok(())
}

pub async fn upload(mut payload: Multipart) -> Result<HttpResponse, Error> {
    // iterate over the multipart fields and save each one to a file
    while let Ok(Some(field)) = payload.try_next().await {
        save_field(field).await?;
    }

    Ok(HttpResponse::Ok().into())
}

pub fn init<T>(app: App<T>) -> App<T>
where
    T: ServiceFactory<ServiceRequest, Config = (), Response = actix_web::dev::ServiceResponse, Error = actix_web::Error, InitError = ()>,
{
    app.route("/upload", web::post().to(upload))
}
