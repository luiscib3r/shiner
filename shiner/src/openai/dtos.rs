use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize};
use typed_builder::TypedBuilder;

fn deserialize_unix_timestamp<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let timestamp = u64::deserialize(deserializer)?;

    // Si el timestamp está en segundos (formato común de UNIX)
    let datetime = DateTime::from_timestamp(timestamp as i64, 0)
        .ok_or_else(|| serde::de::Error::custom("invalid timestamp"))?;

    Ok(datetime)
}

fn default_model() -> String {
    "gpt-image-1".into()
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Quality {
    Low,
    Medium,
    High,
    Auto,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Size {
    #[serde(rename = "1024x1024")]
    Square,
    #[serde(rename = "1536x1024")]
    Landscape,
    #[serde(rename = "1024x1536")]
    Portrait,
    Auto,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Format {
    Png,
    Webp,
    Jpeg,
}

#[derive(Serialize, Deserialize, Debug, Clone, TypedBuilder)]
pub struct ImageRequest {
    #[serde(default = "default_model")]
    #[builder(setter(into), default = default_model())]
    pub model: String,

    #[builder(setter(into))]
    pub prompt: String,

    #[builder(default, setter(into))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<Quality>,

    #[builder(default, setter(into))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<Size>,

    #[builder(default, setter(into))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_format: Option<Format>,

    #[builder(default, setter(into))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_compression: Option<u8>,
}

impl From<&ImageRequest> for reqwest::multipart::Form {
    fn from(req: &ImageRequest) -> Self {
        Self::new()
            .text("model", req.model.clone())
            .text("prompt", req.prompt.clone())
            .text(
                "quality",
                req.quality
                    .as_ref()
                    .map(|q| match q {
                        Quality::Low => "low",
                        Quality::Medium => "medium",
                        Quality::High => "high",
                        Quality::Auto => "auto",
                    })
                    .unwrap_or("auto")
                    .to_string(),
            )
            .text(
                "size",
                req.size
                    .as_ref()
                    .map(|s| match s {
                        Size::Square => "1024x1024",
                        Size::Landscape => "1536x1024",
                        Size::Portrait => "1024x1536",
                        Size::Auto => "auto",
                    })
                    .unwrap_or("auto")
                    .to_string(),
            )
            .text(
                "output_format",
                req.output_format
                    .as_ref()
                    .map(|f| match f {
                        Format::Png => "png",
                        Format::Webp => "webp",
                        Format::Jpeg => "jpeg",
                    })
                    .unwrap_or("png")
                    .to_string(),
            )
            .text(
                "output_compression",
                req.output_compression
                    .map(|c| c.to_string())
                    .unwrap_or_else(|| "0".to_string()),
            )
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ImageResponse {
    #[serde(deserialize_with = "deserialize_unix_timestamp")]
    pub created: DateTime<Utc>,
    pub data: Vec<ImageData>,
    pub usage: Usage,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ImageData {
    pub b64_json: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Usage {
    pub input_tokens: u64,
    pub output_tokens: u64,
    pub total_tokens: u64,
    pub input_tokens_details: InputTokensDetails,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InputTokensDetails {
    pub image_tokens: u64,
    pub text_tokens: u64,
}
