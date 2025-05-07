use std::{fs::File, io::Write, path::Path};

use base64::{Engine, engine::general_purpose::STANDARD as Base64};
use dotenvy::dotenv;
use shiner::openai::{
    api::*,
    dtos::{Format, ImageRequest, Quality, Size},
};

#[tokio::main]
async fn main() {
    dotenv().unwrap_or_default();

    let auth = ApiAuth::builder().build();
    let images = ImagesAPI::builder().auth(auth).build();

    let request = ImageRequest::builder()
        .prompt("A futuristic city skyline at sunset")
        .output_format(Format::Webp)
        .quality(Quality::Low)
        .size(Size::Portrait)
        .build();

    match serde_json::to_string_pretty(&request) {
        Ok(json) => println!("Request JSON:\n{}", json),
        Err(e) => eprintln!("Error serializing request: {}", e),
    }

    let result = images.generation(&request).await;

    match result {
        Ok(response) => {
            println!("Generated image URL: {:?}", &response);

            if let Some(image_data) = response.data.first() {
                // Decodificar base64
                match Base64.decode(&image_data.b64_json) {
                    Ok(image_bytes) => {
                        // Crear un nombre de archivo basado en la fecha/hora
                        let filename = format!(
                            "generated_image_{}.webp",
                            chrono::Utc::now().format("%Y%m%d_%H%M%S")
                        );

                        // Guardar en archivo
                        let path = Path::new(&filename);
                        match File::create(&path).and_then(|mut file| file.write_all(&image_bytes))
                        {
                            Ok(_) => println!("Image saved to {}", filename),
                            Err(e) => eprintln!("Error saving image: {}", e),
                        }
                    }
                    Err(e) => eprintln!("Error decoding base64: {}", e),
                }
            } else {
                eprintln!("No image data received in the response");
            }
        }
        Err(e) => {
            eprintln!("Error generating image: {}", e);
        }
    }
}
