use axum::{
    extract::{Multipart, Path},
    http::StatusCode,
    response::IntoResponse,
    routing::post,
    Router,
};
use std::{fs::create_dir_all, path::Path as FilePath};

pub async fn upload_image_handler(
    Path(image_name): Path<String>,
    mut multipart: Multipart,
) -> impl IntoResponse {
    let images_folder = "./images";
    if !FilePath::new(images_folder).exists() {
        create_dir_all(images_folder).unwrap();
    }

    while let Some(field) = multipart.next_field().await.unwrap() {
        if let Some(filename) = field.file_name() {
            if filename.ends_with(".webp") {
                let filepath = format!("{}/{}.webp", images_folder, image_name);
                let data = field.bytes().await.unwrap();
                tokio::fs::write(filepath, data).await.unwrap();
                return StatusCode::OK.into_response();
            } else {
                return (StatusCode::BAD_REQUEST, "Only .webp files are allowed").into_response();
            }
        }
    }

    (StatusCode::BAD_REQUEST, "No valid file provided").into_response()
}

pub fn admin_routes() -> Router {
    Router::new().route("/admin/upload_image/:image_name", post(upload_image_handler))
}

