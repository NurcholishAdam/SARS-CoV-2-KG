// crates/limit-reflection/src/api.rs
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

use crate::engine::{ReflectionEngine, ReflectionResult};
use crate::govern::{ReflectionGovernance, QualityReport};
use crate::model::MetaCognitiveInsights;

/// API state
pub struct ReflectionApiState {
    pub engine: ReflectionEngine,
    pub governance: ReflectionGovernance,
}

impl ReflectionApiState {
    pub fn new() -> Self {
        Self {
            engine: ReflectionEngine::new(3),
            governance: ReflectionGovernance::default_rules(),
        }
    }
}

/// Create reflection API router
pub fn create_router() -> Router {
    let state = Arc::new(RwLock::new(ReflectionApiState::new()));

    Router::new()
        .route("/health", get(health_check))
        .route("/reflect", post(reflect_on_query))
        .route("/deep-reflect", post(deep_reflect))
        .route("/insights", get(get_insights))
        .route("/suggestions", get(get_suggestions))
        .route("/quality", get(check_quality))
        .with_state(state)
}

async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "healthy".to_string(),
        version: "2.4.1".to_string(),
        service: "limit-reflection".to_string(),
    })
}

async fn reflect_on_query(
    State(state): State<Arc<RwLock<ReflectionApiState>>>,
    Json(request): Json<ReflectRequest>,
) -> Result<Json<ReflectResponse>, StatusCode> {
    let state = state.read().await;

    let result = state
        .engine
        .reflect_on_query(&request.query)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ReflectResponse {
        steps_count: result.steps.len(),
        final_confidence: result.final_confidence,
        insights: result.insights,
    }))
}

async fn deep_reflect(
    State(state): State<Arc<RwLock<ReflectionApiState>>>,
    Json(request): Json<ReflectRequest>,
) -> Result<Json<DeepReflectResponse>, StatusCode> {
    let state = state.read().await;

    let result = state
        .engine
        .deep_reflect(&request.query)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(DeepReflectResponse {
        layers_count: result.layers.len(),
        final_depth: result.final_depth,
        final_confidence: result.layers.last().map(|l| l.final_confidence).unwrap_or(0.0),
    }))
}

async fn get_insights(
    State(state): State<Arc<RwLock<ReflectionApiState>>>,
) -> Json<MetaCognitiveInsights> {
    let state = state.read().await;
    Json(state.engine.get_insights())
}

async fn get_suggestions(
    State(state): State<Arc<RwLock<ReflectionApiState>>>,
) -> Json<SuggestionsResponse> {
    let state = state.read().await;
    let suggestions = state.engine.get_suggestions();
    let approved = state.governance.approve_suggestions(&suggestions);

    Json(SuggestionsResponse {
        total: suggestions.len(),
        approved: approved.len(),
        suggestions: approved,
    })
}

async fn check_quality(
    State(state): State<Arc<RwLock<ReflectionApiState>>>,
) -> Json<QualityReport> {
    let state = state.read().await;
    let model = state.engine.model.read().unwrap();
    Json(state.governance.check_quality(&*model))
}

#[derive(Debug, Serialize)]
struct HealthResponse {
    status: String,
    version: String,
    service: String,
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
    approved: usize,
    suggestions: Vec<crate::govern::ApprovedSuggestion>,
}
