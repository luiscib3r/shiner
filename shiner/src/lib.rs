pub mod openai;

pub async fn test_async() -> Result<String, Box<dyn std::error::Error>> {
    Ok("Hello, world!".to_string())
}
