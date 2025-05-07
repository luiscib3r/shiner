use serde::Serialize;

const UP: &str = "UP";

#[derive(Serialize)]
pub struct HealthCheck {
    pub status: &'static str,
}

impl HealthCheck {
    pub fn new() -> Self {
        Self { status: UP }
    }
}
