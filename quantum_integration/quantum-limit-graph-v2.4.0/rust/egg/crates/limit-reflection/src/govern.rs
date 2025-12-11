// crates/limit-reflection/src/govern.rs
use serde::{Serialize, Deserialize};
use crate::model::{ReflectionModel, Suggestion};
use limit_hub::governance::{GovernanceRules, ValidationResult};

/// Governance for meta-cognitive reasoning
pub struct ReflectionGovernance {
    rules: ReflectionRules,
}

impl ReflectionGovernance {
    pub fn new(rules: ReflectionRules) -> Self {
        Self { rules }
    }

    pub fn default_rules() -> Self {
        Self {
            rules: ReflectionRules::default(),
        }
    }

    /// Validate reflection model against governance rules
    pub fn validate_reflection(&self, model: &ReflectionModel) -> ReflectionValidation {
        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        // Check minimum confidence
        let avg_confidence = if !model.confidence_history.is_empty() {
            model.confidence_history.iter().sum::<f32>() / model.confidence_history.len() as f32
        } else {
            0.0
        };

        if avg_confidence < self.rules.min_average_confidence {
            errors.push(format!(
                "Average confidence {:.2} below minimum {:.2}",
                avg_confidence, self.rules.min_average_confidence
            ));
        }

        // Check error rate
        let total_errors: usize = model.error_patterns.values().sum();
        let error_rate = if !model.reasoning_trace.is_empty() {
            total_errors as f32 / model.reasoning_trace.len() as f32
        } else {
            0.0
        };

        if error_rate > self.rules.max_error_rate {
            warnings.push(format!(
                "Error rate {:.2} exceeds maximum {:.2}",
                error_rate, self.rules.max_error_rate
            ));
        }

        // Check reasoning depth
        if model.reasoning_trace.len() < self.rules.min_reasoning_steps {
            warnings.push(format!(
                "Reasoning steps {} below minimum {}",
                model.reasoning_trace.len(),
                self.rules.min_reasoning_steps
            ));
        }

        ReflectionValidation {
            valid: errors.is_empty(),
            errors,
            warnings,
            requires_review: !warnings.is_empty() || avg_confidence < 0.7,
        }
    }

    /// Approve or reject suggestions
    pub fn approve_suggestions(&self, suggestions: &[Suggestion]) -> Vec<ApprovedSuggestion> {
        suggestions
            .iter()
            .filter(|s| s.priority >= self.rules.min_suggestion_priority)
            .map(|s| ApprovedSuggestion {
                suggestion: s.clone(),
                approved: true,
                reason: "Meets priority threshold".to_string(),
            })
            .collect()
    }

    /// Check if reflection meets quality standards
    pub fn check_quality(&self, model: &ReflectionModel) -> QualityReport {
        let insights = model.get_insights();

        let confidence_score = insights.average_confidence;
        let error_score = 1.0 - (insights.total_errors as f32 / (insights.total_steps as f32 + 1.0));
        let completeness_score = (insights.total_steps as f32 / self.rules.min_reasoning_steps as f32).min(1.0);

        let overall_quality = (confidence_score + error_score + completeness_score) / 3.0;

        QualityReport {
            overall_quality,
            confidence_score,
            error_score,
            completeness_score,
            meets_standards: overall_quality >= self.rules.min_quality_score,
        }
    }
}

/// Reflection governance rules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReflectionRules {
    pub min_average_confidence: f32,
    pub max_error_rate: f32,
    pub min_reasoning_steps: usize,
    pub min_suggestion_priority: f32,
    pub min_quality_score: f32,
}

impl Default for ReflectionRules {
    fn default() -> Self {
        Self {
            min_average_confidence: 0.7,
            max_error_rate: 0.2,
            min_reasoning_steps: 3,
            min_suggestion_priority: 0.6,
            min_quality_score: 0.75,
        }
    }
}

/// Reflection validation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReflectionValidation {
    pub valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub requires_review: bool,
}

/// Approved suggestion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovedSuggestion {
    pub suggestion: Suggestion,
    pub approved: bool,
    pub reason: String,
}

/// Quality report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityReport {
    pub overall_quality: f32,
    pub confidence_score: f32,
    pub error_score: f32,
    pub completeness_score: f32,
    pub meets_standards: bool,
}
