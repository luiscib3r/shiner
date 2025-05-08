use anyhow::Result;
use shiner::openai::{
    api::{ApiAuth, ImagesAPI},
    dtos::{ImageRequest, Size},
};
use std::sync::Arc;

pub struct ImageGenerator {
    images_api: Arc<ImagesAPI>,
}

impl ImageGenerator {
    pub fn new(api_key: String) -> Self {
        let auth = ApiAuth::builder().api_key(api_key).build();
        let images_api = ImagesAPI::builder().auth(auth).build();
        ImageGenerator {
            images_api: Arc::new(images_api),
        }
    }
}

impl ImageGenerator {
    pub async fn run(&self, prompt: String) -> Result<Vec<u8>> {
        let request = ImageRequest::builder()
            .prompt(prompt)
            .size(Size::Portrait)
            .build();

        let image_bytes = self.images_api.generate(&request).await?;
        Ok(image_bytes)
    }
}
