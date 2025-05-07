use axum::{Router, routing::post};
use shiner::openai::api::{ApiAuth, ImagesAPI};

use super::handlers::{edit, generate, generation};

pub fn new() -> Router {
    let auth = ApiAuth::builder().build();
    let images_api = ImagesAPI::builder().auth(auth).build();

    let router = Router::new()
        .route("/generate", post(generate))
        .route("/generation", post(generation))
        .route("/edit", post(edit))
        .with_state(images_api);

    Router::new().nest("/images", router)
}
