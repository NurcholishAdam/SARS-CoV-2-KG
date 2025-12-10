// crates/limit-benchmark/src/harness.rs
use crate::metrics::{GraphMetrics, QueryMetrics};
use crate::multi::{MultiIntentQuery, MultiIntentResult};
use serde::{Serialize, Deserialize};
use std::time::Instant;

/// Benchmark harness for SARS-CoV-2 knowledge graph
pub struct BenchmarkHarness {
    pub name: String,
    pub queries: Vec<MultiIntentQuery>,
    pub results: Vec<BenchmarkResult>,
}

impl BenchmarkHarness {
    pub fn new(name: String) -> Self {
        Self {
            name,
            queries: vec![],
            results: vec![],
        }
    }

    pub fn add_query(&mut self, query: MultiIntentQuery) {
        self.queries.push(query);
    }

    pub fn run<F>(&mut self, executor: F) -> HarnessReport
    where
        F: Fn(&MultiIntentQuery) -> MultiIntentResult,
    {
        self.results.clear();
        let start = Instant::now();

        for query in &self.queries {
            let query_start = Instant::now();
            let result = executor(query);
            let latency = query_start.elapsed().as_millis() as f64;

            self.results.push(BenchmarkResult {
                query_id: query.id.clone(),
                latency_ms: latency,
                success: result.success,
                intent_coverage: result.intent_coverage,
            });
        }

        let total_time = start.elapsed().as_millis() as f64;
        self.generate_report(total_time)
    }

    fn generate_report(&self, total_time_ms: f64) -> HarnessReport {
        let total = self.results.len();
        let successful = self.results.iter().filter(|r| r.success).count();
        let avg_latency = self.results.iter().map(|r| r.latency_ms).sum::<f64>() / total as f64;
        let avg_coverage = self.results.iter().map(|r| r.intent_coverage).sum::<f32>() / total as f32;

        HarnessReport {
            benchmark_name: self.name.clone(),
            total_queries: total,
            successful_queries: successful,
            total_time_ms,
            avg_latency_ms: avg_latency,
            avg_intent_coverage: avg_coverage,
            throughput_qps: (total as f64 / total_time_ms) * 1000.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkResult {
    pub query_id: String,
    pub latency_ms: f64,
    pub success: bool,
    pub intent_coverage: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HarnessReport {
    pub benchmark_name: String,
    pub total_queries: usize,
    pub successful_queries: usize,
    pub total_time_ms: f64,
    pub avg_latency_ms: f64,
    pub avg_intent_coverage: f32,
    pub throughput_qps: f64,
}
