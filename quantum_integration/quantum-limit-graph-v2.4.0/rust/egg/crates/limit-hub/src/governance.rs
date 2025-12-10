// crates/limit-hub/src/governance.rs
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

/// Governance rules for open-source hub
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceRules {
    pub min_confidence: f32,
    pub min_provenance_count: usize,
    pub allowed_sources: Vec<String>,
    pub quality_threshold: f32,
    pub review_required: bool,
}

impl GovernanceRules {
    pub fn default_rules() -> Self {
        Self {
            min_confidence: 0.7,
            min_provenance_count: 2,
            allowed_sources: vec![
                "PubMed".to_string(),
                "bioRxiv".to_string(),
                "medRxiv".to_string(),
            ],
            quality_threshold: 0.8,
            review_required: true,
        }
    }

    pub fn validate_submission(&self, submission: &Submission) -> ValidationResult {
        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        if submission.confidence < self.min_confidence {
            errors.push(format!(
                "Confidence {} below minimum {}",
                submission.confidence, self.min_confidence
            ));
        }

        if submission.provenance.len() < self.min_provenance_count {
            errors.push(format!(
                "Provenance count {} below minimum {}",
                submission.provenance.len(),
                self.min_provenance_count
            ));
        }

        for source in &submission.provenance {
            if !self.allowed_sources.contains(source) {
                warnings.push(format!("Source {} not in allowed list", source));
            }
        }

        if submission.quality_score < self.quality_threshold {
            warnings.push(format!(
                "Quality score {} below threshold {}",
                submission.quality_score, self.quality_threshold
            ));
        }

        ValidationResult {
            valid: errors.is_empty(),
            errors,
            warnings,
            requires_review: self.review_required || !warnings.is_empty(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Submission {
    pub id: String,
    pub content: String,
    pub confidence: f32,
    pub provenance: Vec<String>,
    pub quality_score: f32,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub requires_review: bool,
}
