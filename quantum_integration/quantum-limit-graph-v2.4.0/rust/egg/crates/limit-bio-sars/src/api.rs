// crates/limit-bio-sars/src/api.rs
use axum::{Router, routing::get, extract::State, Json};
use std::sync::{Arc, Mutex};
use crate::graph::BioGraph;

#[derive(Clone)]
pub struct AppState { pub graphs: Arc<Mutex<Vec<BioGraph>>> }

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/bio/graphs", get(list_graphs))
        .route("/bio/graph/counts", get(counts))
        .with_state(state)
}

async fn list_graphs(State(state): State<AppState>) -> Json<Vec<String>> {
    let g = state.graphs.lock().unwrap();
    Json(g.iter().map(|x| x.id.to_string()).collect())
}

#[derive(serde::Serialize)]
struct Counts { proteins: usize, receptors: usize, variants: usize, therapies: usize, edges: usize }

async fn counts(State(state): State<AppState>) -> Json<Option<Counts>> {
    let g = state.graphs.lock().unwrap();
    Json(g.first().map(|x| Counts {
        proteins: x.proteins.len(),
        receptors: x.receptors.len(),
        variants: x.variants.len(),
        therapies: x.therapies.len(),
        edges: x.edges.len(),
    }))
}
