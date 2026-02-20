use axum::extract::{Json, Path, Query};

#[axum::debug_handler]
pub async fn check_health() {}
