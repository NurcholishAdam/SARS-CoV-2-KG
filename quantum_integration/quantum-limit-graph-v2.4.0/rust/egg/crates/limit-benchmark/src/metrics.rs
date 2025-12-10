// crates/limit-benchmark/src/metrics.rs
use serde::{Serialize, Deserialize};

/// Comprehensive metrics for SARS-CoV-2 knowledge graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphMetrics {
    pub node_count: usize,
    pub edge_count: usize,
    pub avg_degree: f32,
    pub density: f32,
    pub coverage_score: f32,
    pub provenance_score: f32,
}

impl GraphMetrics {
    pub fn compute(node_count: usize, edge_count: usize) -> Self {
        let avg_degree = if node_count > 0 {
            (2 * edge_count) as f32 / node_count as f32
        } else {
            0.0
        };

        let max_edges = node_count * (node_count - 1) / 2;
        let density = if max_edges > 0 {
            edge_count as f32 / max_edges as f32
        } else {
            0.0
        };

        Self {
            node_count,
            edge_count,
            avg_degree,
            density,
            coverage_score: 0.0,
            provenance_score: 0.0,
        }
    }

    pub fn with_coverage(mut self, coverage: f32) -> Self {
        self.coverage_score = coverage;
        self
    }

    pub fn with_provenance(mut self, provenance: f32) -> Self {
        self.provenance_score = provenance;
        self
    }
}

/// Query performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryMetrics {
    pub latency_ms: f64,
    pub throughput_qps: f64,
    pub accuracy: f32,
    pub recall: f32,
    pub precision: f32,
}

impl QueryMetrics {
    pub fn new(latency_ms: f64, accuracy: f32, recall: f32, precision: f32) -> Self {
        let throughput_qps = if latency_ms > 0.0 {
            1000.0 / latency_ms
        } else {
            0.0
        };

        Self {
            latency_ms,
            throughput_qps,
            accuracy,
            recall,
            precision,
        }
    }

    pub fn f1_score(&self) -> f32 {
        if self.precision + self.recall > 0.0 {
            2.0 * (self.precision * self.recall) / (self.precision + self.recall)
        } else {
            0.0
        }
    }
}
