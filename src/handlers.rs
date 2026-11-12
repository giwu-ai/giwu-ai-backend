use crate::agents::{run_compare_agent, run_job_scam_agent, run_text_agent};
use crate::models::{AppState, InspectionResult};
use axum::{
    extract::{Multipart, State},
    http::StatusCode,
    Json,
};

pub async fn verify_product_handler(
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Result<Json<InspectionResult>, StatusCode> {
    let mut description = String::new();
    let mut image_bytes = Vec::new();

    while let Ok(Some(field)) = multipart.next_field().await {
        let name = field.name().unwrap_or("").to_string();
        if name == "description" || name == "listing_description" {
            description = field.text().await.unwrap_or_default();
        } else if name == "image" {
            image_bytes = field.bytes().await.unwrap_or_default().to_vec();
        }
    }

    // Route to Text Agent or General Job Scam Image Agent synchronously
    let result = if !description.is_empty() && description != "Text Scam Analysis" {
        run_text_agent(&state.gemini_client, &description).await
    } else {
        run_job_scam_agent(&state.gemini_client, &image_bytes).await
    };

    match result {
        Ok(res) => Ok(Json(res)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn compare_product_handler(
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Result<Json<InspectionResult>, StatusCode> {
    let mut reference_id = String::new();
    let mut image_bytes = Vec::new();

    while let Ok(Some(field)) = multipart.next_field().await {
        let name = field.name().unwrap_or("").to_string();
        if name == "reference_id" {
            reference_id = field.text().await.unwrap_or_default();
        } else if name == "image" {
            image_bytes = field.bytes().await.unwrap_or_default().to_vec();
        }
    }

    // Route to Catalog Compare Agent synchronously
    match run_compare_agent(&state.gemini_client, &reference_id, &image_bytes).await {
        Ok(res) => Ok(Json(res)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

