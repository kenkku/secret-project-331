/*!
Handlers for HTTP requests to `/api/v0/files`.

*/
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

pub use crate::domain::{authorization::AuthorizationToken, models_requests::UploadClaim};
use crate::prelude::*;
use actix_files::NamedFile;
use futures::{StreamExt, TryStreamExt};
use headless_lms_utils::file_store::file_utils;
use tokio::fs::read;
/**

GET `/api/v0/files/\*` Redirects the request to a file storage service.

This is meant for redirecting requests to appropriate storage services.
This approach decouples the storage mechanism from the urls.
Redirection is done with HTTP status 302 Found and it has a max
age of 5 minutes.

Redirects to local file handler in development and to a service in production.


# Example

`GET /api/v0/files/organizations/1b89e57e-8b57-42f2-9fed-c7a6736e3eec/courses/d86cf910-4d26-40e9-8c9c-1cc35294fdbb/images/nNQbVax81fH4SLCXuQ9NrOWtqfHT6x.jpg`

Response headers:
```text
< HTTP/1.1 302 Found
< Date: Mon, 26 Apr 2021 10:38:09 GMT
< Content-Length: 0
< Connection: keep-alive
< cache-control: max-age=300, private
< location: /api/v0/files/uploads/organizations/1b89e57e-8b57-42f2-9fed-c7a6736e3eec/courses/d86cf910-4d26-40e9-8c9c-1cc35294fdbb/images/nNQbVax81fH4SLCXuQ9NrOWtqfHT6x.jpg
```

*/
#[instrument(skip(file_store))]
#[allow(clippy::async_yields_async)]
async fn redirect_to_storage_service(
    tail: web::Path<String>,
    file_store: web::Data<dyn FileStore>,
) -> HttpResponse {
    let inner = tail.into_inner();
    let tail_path = Path::new(&inner);

    match file_store.get_direct_download_url(tail_path).await {
        Ok(res) => HttpResponse::Found()
            .append_header(("location", res))
            .append_header(("cache-control", "max-age=300, private"))
            .finish(),
        Err(e) => {
            error!("Could not get file {:?}", e);
            HttpResponse::NotFound()
                .append_header(("cache-control", "max-age=300, private"))
                .finish()
        }
    }
}

/**
GET `/api/v0/files/uploads/\*`
Serve local uploaded file, mostly for development.

# Example

`GET /api/v0/files/uploads/organizations/1b89e57e-8b57-42f2-9fed-c7a6736e3eec/courses/d86cf910-4d26-40e9-8c9c-1cc35294fdbb/images/nNQbVax81fH4SLCXuQ9NrOWtqfHT6x.jpg`

Result:

The file.
*/
#[instrument(skip(req))]
async fn serve_upload(req: HttpRequest, pool: web::Data<PgPool>) -> ControllerResult<HttpResponse> {
    // TODO: replace this whole function with the actix_files::Files service once it works with the used actix version.
    let base_folder = Path::new("uploads");
    let relative_path = PathBuf::from(req.match_info().query("tail"));
    let path = base_folder.join(relative_path);

    let named_file = NamedFile::open(path).map_err(|_e| {
        ControllerError::new(
            ControllerErrorType::NotFound,
            "File not found".to_string(),
            None,
        )
    })?;
    let path = named_file.path();
    let contents = read(path).await.map_err(|_e| {
        ControllerError::new(
            ControllerErrorType::InternalServerError,
            "Could not read file".to_string(),
            None,
        )
    })?;

    let extension = path.extension().map(|o| o.to_string_lossy().to_string());
    let mut mime_type = None;
    if let Some(ext_string) = extension {
        mime_type = match ext_string.as_str() {
            "jpg" => Some("image/jpg"),
            "png" => Some("image/png"),
            "svg" => Some("image/svg+xml"),
            "webp" => Some("image/webp"),
            "gif" => Some("image/gif"),
            _ => None,
        };
    }
    let mut response = HttpResponse::Ok();
    if let Some(m) = mime_type {
        response.append_header(("content-type", m));
    }

    // this endpoint is only used for development
    let token = skip_authorize()?;
    token.authorized_ok(response.body(contents))
}

/**
POST `/api/v0/files/:exercise_service_slug`
Used to upload data from exercise service iframes.

# Returns
The randomly generated path to the uploaded file.
*/
#[instrument(skip(payload, file_store))]
#[generated_doc]
async fn upload_from_exercise_service(
    exercise_service_slug: web::Path<String>,
    payload: Multipart,
    file_store: web::Data<dyn FileStore>,
    upload_claim: Result<UploadClaim<'static>, ControllerError>,
) -> ControllerResult<web::Json<HashMap<String, String>>> {
    // accessed from exercise services, can't authenticate using login,
    // the upload claim is used to verify requests instead
    let token = skip_authorize()?;

    // the playground uses the special "playground" slug to upload temporary files
    if exercise_service_slug.as_str() != "playground" {
        // non-playground uploads require a valid upload claim
        let upload_claim = upload_claim?;
        if upload_claim.exercise_service_slug() != exercise_service_slug.as_ref() {
            // upload claim's exercise type doesn't match the upload url
            return Err(ControllerError::new(
                ControllerErrorType::BadRequest,
                "Exercise service slug did not match upload claim".to_string(),
                None,
            ));
        }
    }

    let mut paths = HashMap::new();
    if let Err(outer_err) = upload_from_exercise_service_inner(
        exercise_service_slug.as_str(),
        payload,
        file_store.as_ref(),
        &mut paths,
    )
    .await
    {
        // something went wrong while uploading the files, try to delete leftovers
        for path in paths.values() {
            if let Err(err) = file_store.delete(Path::new(path)).await {
                error!("Failed to delete file '{path}' during cleanup: {err}")
            }
        }
        return Err(outer_err);
    }

    token.authorized_ok(web::Json(paths))
}

// Wraps the uploading logic
async fn upload_from_exercise_service_inner(
    exercise_service_slug: &str,
    mut payload: Multipart,
    file_store: &dyn FileStore,
    paths: &mut HashMap<String, String>,
) -> Result<(), ControllerError> {
    while let Some(item) = payload.next().await {
        let field = item.unwrap();
        let field_name = field.name().to_string();

        let random_filename = file_utils::random_filename();
        let path = format!("{exercise_service_slug}/{random_filename}");

        // todo: convert archives into a uniform format
        file_store
            .upload_stream(
                Path::new(&path),
                field.map_err(Into::into).boxed_local(),
                "application/octet-stream",
            )
            .await?;

        paths.insert(field_name, path.clone());
    }
    Ok(())
}

/**
Add a route for each controller in this module.

The name starts with an underline in order to appear before other functions in the module documentation.

We add the routes by calling the route method instead of using the route annotations because this method preserves the function signatures for documentation.
*/
pub fn _add_routes(cfg: &mut ServiceConfig) {
    cfg.route("/uploads/{tail:.*}", web::get().to(serve_upload))
        .route(
            "/{exercise_service_slug}",
            web::post().to(upload_from_exercise_service),
        )
        .route("{tail:.*}", web::get().to(redirect_to_storage_service));
}
