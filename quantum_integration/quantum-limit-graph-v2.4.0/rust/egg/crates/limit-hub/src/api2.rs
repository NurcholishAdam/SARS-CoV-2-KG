// crates/limit-hub/src/api2.rs
// Combined API with Hub + Reflection integration

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
use crate::state::CombinedHubState;
use limit_reflection::{MetaCognitiveInsights, ReflectionResult};

/// Create combined Hub + Reflection API router
pub fn create_combined_router() -> Router {
    let state = Arc::new(RwLock::new(CombinedHubState::new()));

    Router::new()
        // Hub endpoints
        .route("/health", get(health_check))
        .route("/submit", post(submit_data))
        .route("/submissions", get(list_submissions))
        .route("/submissions/:id", get(get_submission))
        .route("/validate", post(validate_submission))
        // Reflection endpoints
        .route("/reflect", post(reflect_on_query))
        .route("/deep-reflect", post(deep_reflect))
        .route("/insights", get(get_insights))
        .route("/suggestions", get(get_suggestions))
        // Combined endpoints
        .route("/reflect-with-evidence", post(reflect_with_evidence))
        .with_state(state)
}

// ============================================================================
// Hub Endpoints
// ============================================================================

async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "healthy".to_string(),
        version: "2.4.1".to_string(),
        service: "limit-hub-combined".to_string(),
    })
}

async fn submit_data(
    State(state): State<Arc<RwLock<CombinedHubState>>>,
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
    State(state): State<Arc<RwLock<CombinedHubState>>>,
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
    State(state): State<Arc<RwLock<CombinedHubState>>>,
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
    State(state): State<Arc<RwLock<CombinedHubState>>>,
    Json(submission): Json<Submission>,
) -> Json<ValidationResult> {
    let state = state.read().await;
    Json(state.governance.validate_submission(&submission))
}

// ============================================================================
// Reflection Endpoints
// ============================================================================

async fn reflect_on_query(
    State(state): State<Arc<RwLock<CombinedHubState>>>,
    Json(request): Json<ReflectRequest>,
) -> Result<Json<ReflectResponse>, StatusCode> {
    let state = state.read().await;

    let result = state
        .reflect_with_context(&request.query)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ReflectResponse {
        steps_count: result.steps.len(),
        final_confidence: result.final_confidence,
        insights: result.insights,
    }))
}

async fn deep_reflect(
    State(state): State<Arc<RwLock<CombinedHubState>>>,
    Json(request): Json<ReflectRequest>,
) -> Result<Json<DeepReflectResponse>, StatusCode> {
    let state = state.read().await;

    let result = state
        .reflection_engine
        .deep_reflect(&request.query)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(DeepReflectResponse {
        layers_count: result.layers.len(),
        final_depth: result.final_depth,
        final_confidence: result.layers.last().map(|l| l.final_confidence).unwrap_or(0.0),
    }))
}

async fn get_insights(
    State(state): State<Arc<RwLock<CombinedHubState>>>,
) -> Json<MetaCognitiveInsights> {
    let state = state.read().await;
    Json(state.get_reflection_insights())
}

async fn get_suggestions(
    State(state): State<Arc<RwLock<CombinedHubState>>>,
) -> Json<SuggestionsResponse> {
    let state = state.read().await;
    let suggestions = state.reflection_engine.get_suggestions();

    Json(SuggestionsResponse {
        total: suggestions.len(),
        suggestions,
    })
}

// ============================================================================
// Combined Endpoints (Evidence + Reflection)
// ============================================================================

async fn reflect_with_evidence(
    State(state): State<Arc<RwLock<CombinedHubState>>>,
    Json(request): Json<ReflectWithEvidenceRequest>,
) -> Result<Json<ReflectWithEvidenceResponse>, StatusCode> {
    let state_guard = state.read().await;

    // Get relevant submissions as evidence
    let evidence: Vec<EvidenceItem> = state_guard
        .submissions
        .iter()
        .filter(|s| s.confidence >= 0.7)
        .take(5)
        .map(|s| EvidenceItem {
            id: s.id.clone(),
            content: s.content.clone(),
            confidence: s.confidence,
            provenance: s.provenance.clone(),
        })
        .collect();

    // Perform reflection with evidence context
    let enriched_query = format!(
        "{} [Evidence items: {}]",
        request.query,
        evidence.len()
    );

    let reflection = state_guard
        .reflection_engine
        .reflect_on_query(&enriched_query)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ReflectWithEvidenceResponse {
        query: request.query,
        evidence,
        reflection: ReflectionSummary {
            steps_count: reflection.steps.len(),
            final_confidence: reflection.final_confidence,
            insights: reflection.insights,
        },
        combined_confidence: (reflection.final_confidence
            + evidence.iter().map(|e| e.confidence).sum::<f32>() / evidence.len() as f32)
            / 2.0,
    }))
}

// ============================================================================
// Request/Response Types
// ============================================================================

#[derive(Debug, Serialize)]
struct HealthResponse {
    status: String,
    version: String,
    service: String,
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

#[derive(Debug, Deserialize)]
struct ReflectRequest {
    query: String,
}

#[derive(Debug, Serialize)]
struct ReflectResponse {
    steps_count: usize,
    final_confidence: f32,
    insights: MetaCognitiveInsights,
}

#[derive(Debug, Serialize)]
struct DeepReflectResponse {
    layers_count: usize,
    final_depth: usize,
    final_confidence: f32,
}

#[derive(Debug, Serialize)]
struct SuggestionsResponse {
    total: usize,
    suggestions: Vec<limit_reflection::Suggestion>,
}

#[derive(Debug, Deserialize)]
struct ReflectWithEvidenceRequest {
    query: String,
}

#[derive(Debug, Serialize)]
struct ReflectWithEvidenceResponse {
    query: String,
    evidence: Vec<EvidenceItem>,
    reflection: ReflectionSummary,
    combined_confidence: f32,
}

#[derive(Debug, Serialize)]
struct EvidenceItem {
    id: String,
    content: String,
    confidence: f32,
    provenance: Vec<String>,
}

#[derive(Debug, Serialize)]
struct ReflectionSummary {
    steps_count: usize,
    final_confidence: f32,
    insights: MetaCognitiveInsights,
}
