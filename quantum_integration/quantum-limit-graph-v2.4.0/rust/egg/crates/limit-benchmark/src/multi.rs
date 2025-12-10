// crates/limit-benchmark/src/multi.rs
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

/// Multi-intent query representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiIntentQuery {
    pub id: String,
    pub intents: Vec<Intent>,
    pub context: HashMap<String, String>,
}

/// Individual intent within a multi-intent query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Intent {
    pub intent_type: IntentType,
    pub query: String,
    pub priority: f32,
    pub domain: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum IntentType {
    Factual,
    Causal,
    Comparative,
    Predictive,
    Exploratory,
}

/// Multi-intent harness for benchmarking
pub struct MultiIntentHarness {
    pub queries: Vec<MultiIntentQuery>,
    pub results: Vec<MultiIntentResult>,
}

impl MultiIntentHarness {
    pub fn new() -> Self {
        Self {
            queries: vec![],
            results: vec![],
        }
    }

    pub fn add_query(&mut self, query: MultiIntentQuery) {
        self.queries.push(query);
    }

    pub fn run_benchmark<F>(&mut self, executor: F) -> BenchmarkSummary
    where
        F: Fn(&MultiIntentQuery) -> MultiIntentResult,
    {
        self.results.clear();
        for query in &self.queries {
            let result = executor(query);
            self.results.push(result);
        }
        self.compute_summary()
    }

    fn compute_summary(&self) -> BenchmarkSummary {
        let total = self.results.len();
        let successful = self.results.iter().filter(|r| r.success).count();
        let avg_latency = self.results.iter().map(|r| r.latency_ms).sum::<f64>() / total as f64;
        let avg_coverage = self.results.iter().map(|r| r.intent_coverage).sum::<f32>() / total as f32;

        BenchmarkSummary {
            total_queries: total,
            successful_queries: successful,
            avg_latency_ms: avg_latency,
            avg_intent_coverage: avg_coverage,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiIntentResult {
    pub query_id: String,
    pub success: bool,
    pub latency_ms: f64,
    pub intent_coverage: f32,
    pub provenance: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkSummary {
    pub total_queries: usize,
    pub successful_queries: usize,
    pub avg_latency_ms: f64,
    pub avg_intent_coverage: f32,
}

impl Default for MultiIntentHarness {
    fn default() -> Self {
        Self::new()
    }
}
