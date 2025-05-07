use typed_builder::TypedBuilder;

use super::dtos::{ImageRequest, ImageResponse};
use anyhow::{Ok, Result};
use base64::{Engine, engine::general_purpose::STANDARD as Base64};

fn load_api_key() -> String {
    std::env::var("OPENAI_API_KEY").unwrap_or_else(|_| {
        panic!("OPENAI_API_KEY key not provided and not found in environment variables.")
    })
}

#[derive(Clone, Debug, TypedBuilder)]
pub struct ApiAuth {
    #[builder(setter(into), default=load_api_key())]
    api_key: String,
}

impl ApiAuth {
    pub fn api_key(&self) -> &str {
        &self.api_key
    }
}

#[derive(Debug, Clone, TypedBuilder)]
pub struct ImagesAPI {
    auth: ApiAuth,

    #[builder(setter(into), default="https://api.openai.com/v1/images".into())]
    base_url: String,

    #[builder(default=reqwest::Client::new())]
    http: reqwest::Client,
}

impl ImagesAPI {
    pub async fn generate(&self, image_request: &ImageRequest) -> Result<Vec<u8>> {
        let response = self.generation(image_request).await?;

        let image_data = response
            .data
            .first()
            .ok_or_else(|| anyhow::anyhow!("No image data received in the response"))?;

        let image_bytes = Base64.decode(&image_data.b64_json)?;

        Ok(image_bytes)
    }

    pub async fn generation(&self, image_request: &ImageRequest) -> Result<ImageResponse> {
        let request = self
            .http
            .post(format!("{}/generations", self.base_url))
            .header("Authorization", format!("Bearer {}", self.auth.api_key()))
            .header("Content-Type", "application/json")
            .json(image_request);

        let response: ImageResponse = request.send().await?.json().await?;

        Ok(response)
    }

    pub async fn edit(
        &self,
        image_request: &ImageRequest,
        images: Vec<(Vec<u8>, String)>,
        mask: Option<(Vec<u8>, String)>,
    ) -> Result<Vec<u8>> {
        let response = self.edits(image_request, images, mask).await?;

        let image_data = response
            .data
            .first()
            .ok_or_else(|| anyhow::anyhow!("No image data received in the response"))?;

        let image_bytes = Base64.decode(&image_data.b64_json)?;

        Ok(image_bytes)
    }

    pub async fn edits(
        &self,
        image_request: &ImageRequest,
        images: Vec<(Vec<u8>, String)>,
        mask: Option<(Vec<u8>, String)>,
    ) -> Result<ImageResponse> {
        let mut form_data: reqwest::multipart::Form = image_request.into();

        for (i, (image_data, mime_type)) in images.iter().enumerate() {
            let extension = mime_type.split('/').nth(1).unwrap_or("png");
            let part = reqwest::multipart::Part::bytes(image_data.clone())
                .file_name(format!("image_{}.{}", i, extension))
                .mime_str(mime_type)?;
            form_data = form_data.part("image[]", part);
        }

        if let Some((mask_data, mime_type)) = mask {
            let extension = mime_type.split('/').nth(1).unwrap_or("png");
            let mask_part = reqwest::multipart::Part::bytes(mask_data)
                .file_name(format!("mask.{}", extension))
                .mime_str(&mime_type.as_str())?;
            form_data = form_data.part("mask", mask_part);
        }

        let request = self
            .http
            .post(format!("{}/edits", self.base_url))
            .header("Authorization", format!("Bearer {}", self.auth.api_key()))
            .multipart(form_data);

        let response: ImageResponse = request.send().await?.json().await?;

        Ok(response)
    }
}
