use serde::Deserialize;
use shiner::openai::dtos::Format;

fn default_quality() -> Quality {
    Quality::Auto
}

fn default_size() -> Size {
    Size::Auto
}

#[derive(Deserialize)]
pub struct ImageRequest {
    pub prompt: String,
    #[serde(default = "default_quality")]
    pub quality: Quality,

    #[serde(default = "default_size")]
    pub size: Size,
}

impl Default for ImageRequest {
    fn default() -> Self {
        Self {
            prompt: String::new(),
            quality: default_quality(),
            size: default_size(),
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Quality {
    Low,
    Medium,
    High,
    Auto,
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Size {
    Square,
    Landscape,
    Portrait,
    Auto,
}

impl From<ImageRequest> for shiner::openai::dtos::ImageRequest {
    fn from(req: ImageRequest) -> Self {
        let quality: shiner::openai::dtos::Quality = req.quality.into();
        let size: shiner::openai::dtos::Size = req.size.into();

        shiner::openai::dtos::ImageRequest::builder()
            .prompt(req.prompt)
            .output_format(Format::Webp)
            .quality(quality)
            .size(size)
            .build()
    }
}

impl From<Quality> for shiner::openai::dtos::Quality {
    fn from(quality: Quality) -> Self {
        match quality {
            Quality::Low => shiner::openai::dtos::Quality::Low,
            Quality::Medium => shiner::openai::dtos::Quality::Medium,
            Quality::High => shiner::openai::dtos::Quality::High,
            Quality::Auto => shiner::openai::dtos::Quality::Auto,
        }
    }
}

impl From<Size> for shiner::openai::dtos::Size {
    fn from(size: Size) -> Self {
        match size {
            Size::Square => shiner::openai::dtos::Size::Square,
            Size::Landscape => shiner::openai::dtos::Size::Landscape,
            Size::Portrait => shiner::openai::dtos::Size::Portrait,
            Size::Auto => shiner::openai::dtos::Size::Auto,
        }
    }
}
