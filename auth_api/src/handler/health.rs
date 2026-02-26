use axum::extract::{Json, Path, Query};

pub async fn check_health() -> Json<String> {}
