use axum::Json;

use super::dtos::HealthCheck;

pub async fn health_check() -> Json<HealthCheck> {
    Json(HealthCheck::new())
}
