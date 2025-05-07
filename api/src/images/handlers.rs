use crate::{
    core::{AppError, AppJson},
    images::dtos::{Quality, Size},
};
use axum::{
    body::Body,
    extract::{Multipart, State},
    response::{Response, Result},
};
use shiner::openai::{api::ImagesAPI, dtos::ImageResponse};

use super::dtos::ImageRequest;

pub async fn generate(
    State(images_api): State<ImagesAPI>,
    AppJson(payload): AppJson<ImageRequest>,
) -> Result<Response, AppError> {
    let image_bytes = images_api.generate(&payload.into()).await?;

    let content_length = image_bytes.len();
    let body = Body::from(image_bytes);
    let response = Response::builder()
        .status(200)
        .header("Content-Type", "image/webp")
        .header("Content-Length", content_length)
        .body(body)
        .expect("Failed to create response");

    Ok(response)
}

pub async fn generation(
    State(images_api): State<ImagesAPI>,
    AppJson(payload): AppJson<ImageRequest>,
) -> Result<AppJson<ImageResponse>, AppError> {
    let image_response = images_api.generation(&payload.into()).await?;
    Ok(AppJson(image_response))
}

pub async fn edit(
    State(images_api): State<ImagesAPI>,
    mut multipart: Multipart,
) -> Result<Response, AppError> {
    let mut request: ImageRequest = Default::default();

    let mut images: Vec<(Vec<u8>, String)> = vec![];
    let mut mask: Option<(Vec<u8>, String)> = None;

    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();

        match name.as_str() {
            "prompt" => {
                let prompt = field.text().await.unwrap();
                request = ImageRequest { prompt, ..request }
            }
            "quality" => {
                let quality_text = field.text().await.unwrap();
                let quality = match quality_text.to_lowercase().as_str() {
                    "low" => Quality::Low,
                    "medium" => Quality::Medium,
                    "high" => Quality::High,
                    _ => Quality::Auto,
                };
                request = ImageRequest { quality, ..request }
            }
            "size" => {
                let size_text = field.text().await.unwrap();
                let size = match size_text.to_lowercase().as_str() {
                    "square" => Size::Square,
                    "landscape" => Size::Landscape,
                    "portrait" => Size::Portrait,
                    _ => Size::Auto,
                };
                request = ImageRequest { size, ..request }
            }
            "image" => {
                let mime_type = field.content_type().unwrap_or("image/png").to_string();
                let image_bytes = field.bytes().await.unwrap();
                images.push((image_bytes.to_vec(), mime_type.to_string()));
            }
            "mask" => {
                let mime_type = field.content_type().unwrap_or("image/png").to_string();
                let mask_bytes = field.bytes().await.unwrap();
                mask = Some((mask_bytes.to_vec(), mime_type.to_string()));
            }
            _ => {}
        }
    }

    let image_response = images_api.edit(&request.into(), images, mask).await?;

    let content_length = image_response.len();
    let body = Body::from(image_response);

    let response = Response::builder()
        .status(200)
        .header("Content-Type", "image/webp")
        .header("Content-Length", content_length)
        .body(body)
        .expect("Failed to create response");

    Ok(response)
}
