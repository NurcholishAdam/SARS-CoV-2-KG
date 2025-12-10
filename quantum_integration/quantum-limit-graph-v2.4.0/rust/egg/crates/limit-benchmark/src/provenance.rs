// crates/limit-benchmark/src/provenance.rs
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Provenance tracking for knowledge graph operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvenanceRecord {
    pub id: Uuid,
    pub operation: String,
    pub timestamp: String,
    pub source: String,
    pub confidence: f32,
    pub metadata: HashMap<String, String>,
}

impl ProvenanceRecord {
    pub fn new(operation: String, source: String, confidence: f32) -> Self {
        Self {
            id: Uuid::new_v4(),
            operation,
            timestamp: chrono::Utc::now().to_rfc3339(),
            source,
            confidence,
            metadata: HashMap::new(),
        }
    }
}

/// Provenance tracker for graph operations
pub struct ProvenanceTracker {
    records: Vec<ProvenanceRecord>,
}

impl ProvenanceTracker {
    pub fn new() -> Self {
        Self { records: vec![] }
    }

    pub fn record(&mut self, operation: String, source: String, confidence: f32) -> Uuid {
        let record = ProvenanceRecord::new(operation, source, confidence);
        let id = record.id;
        self.records.push(record);
        id
    }

    pub fn get_record(&self, id: Uuid) -> Option<&ProvenanceRecord> {
        self.records.iter().find(|r| r.id == id)
    }

    pub fn get_all_records(&self) -> &[ProvenanceRecord] {
        &self.records
    }

    pub fn filter_by_source(&self, source: &str) -> Vec<&ProvenanceRecord> {
        self.records.iter().filter(|r| r.source == source).collect()
    }

    pub fn filter_by_confidence(&self, min_confidence: f32) -> Vec<&ProvenanceRecord> {
        self.records
            .iter()
            .filter(|r| r.confidence >= min_confidence)
            .collect()
    }
}

impl Default for ProvenanceTracker {
    fn default() -> Self {
        Self::new()
    }
}
