// crates/limit-hub/src/api.rs
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::governance::{GovernanceRules, Submission, ValidationResult};

/// Hub API state
pub struct HubState {
    pub governance: GovernanceRules,
    pub submissions: Vec<Submission>,
}

impl HubState {
    pub fn new() -> Self {
        Self {
            governance: GovernanceRules::default_rules(),
            submissions: vec![],
        }
    }
}

/// Create Hub API router
pub fn create_router() -> Router {
    let state = Arc::new(RwLock::new(HubState::new()));

    Router::new()
        .route("/health", get(health_check))
        .route("/submit", post(submit_data))
        .route("/submissions", get(list_submissions))
        .route("/submissions/:id", get(get_submission))
        .route("/validate", post(validate_submission))
        .with_state(state)
}

async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "healthy".to_string(),
        version: "2.4.1".to_string(),
    })
}

async fn submit_data(
    State(state): State<Arc<RwLock<HubState>>>,
    Json(submission): Json<Submission>,
) -> Result<Json<SubmitResponse>, StatusCode> {
    let mut state = state.write().await;
    let validation = state.governance.validate_submission(&submission);

    if !validation.valid {
        return Err(StatusCode::BAD_REQUEST);
    }

    state.submissions.push(submission.clone());

    Ok(Json(SubmitResponse {
        id: submission.id,
        status: "accepted".to_string(),
        validation,
    }))
}

async fn list_submissions(
    State(state): State<Arc<RwLock<HubState>>>,
) -> Json<Vec<SubmissionSummary>> {
    let state = state.read().await;
    let summaries = state
        .submissions
        .iter()
        .map(|s| SubmissionSummary {
            id: s.id.clone(),
            confidence: s.confidence,
            quality_score: s.quality_score,
        })
        .collect();

    Json(summaries)
}

async fn get_submission(
    State(state): State<Arc<RwLock<HubState>>>,
    Path(id): Path<String>,
) -> Result<Json<Submission>, StatusCode> {
    let state = state.read().await;
    state
        .submissions
        .iter()
        .find(|s| s.id == id)
        .cloned()
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

async fn validate_submission(
    State(state): State<Arc<RwLock<HubState>>>,
    Json(submission): Json<Submission>,
) -> Json<ValidationResult> {
    let state = state.read().await;
    Json(state.governance.validate_submission(&submission))
}

#[derive(Debug, Serialize)]
struct HealthResponse {
    status: String,
    version: String,
}

#[derive(Debug, Serialize)]
struct SubmitResponse {
    id: String,
    status: String,
    validation: ValidationResult,
}

#[derive(Debug, Serialize)]
struct SubmissionSummary {
    id: String,
    confidence: f32,
    quality_score: f32,
}
