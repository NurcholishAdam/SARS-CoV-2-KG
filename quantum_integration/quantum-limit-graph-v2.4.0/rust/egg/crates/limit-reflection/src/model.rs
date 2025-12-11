// crates/limit-reflection/src/model.rs
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use std::collections::HashMap;

/// Meta-cognitive reasoning model for self-reflection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReflectionModel {
    pub id: Uuid,
    pub reasoning_trace: Vec<ReasoningStep>,
    pub confidence_history: Vec<f32>,
    pub error_patterns: HashMap<String, usize>,
    pub improvement_suggestions: Vec<Suggestion>,
}

impl ReflectionModel {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            reasoning_trace: vec![],
            confidence_history: vec![],
            error_patterns: HashMap::new(),
            improvement_suggestions: vec![],
        }
    }

    /// Add a reasoning step to the trace
    pub fn add_step(&mut self, step: ReasoningStep) {
        self.confidence_history.push(step.confidence);
        self.reasoning_trace.push(step);
    }

    /// Record an error pattern
    pub fn record_error(&mut self, error_type: String) {
        *self.error_patterns.entry(error_type).or_insert(0) += 1;
    }

    /// Generate improvement suggestions based on patterns
    pub fn generate_suggestions(&mut self) {
        self.improvement_suggestions.clear();

        // Analyze confidence trends
        if self.confidence_history.len() >= 3 {
            let recent: Vec<f32> = self.confidence_history.iter().rev().take(3).copied().collect();
            let avg = recent.iter().sum::<f32>() / recent.len() as f32;

            if avg < 0.6 {
                self.improvement_suggestions.push(Suggestion {
                    id: Uuid::new_v4(),
                    suggestion_type: SuggestionType::IncreaseConfidence,
                    description: "Recent confidence scores are low. Consider additional validation.".to_string(),
                    priority: 0.8,
                });
            }
        }

        // Analyze error patterns
        for (error_type, count) in &self.error_patterns {
            if *count > 3 {
                self.improvement_suggestions.push(Suggestion {
                    id: Uuid::new_v4(),
                    suggestion_type: SuggestionType::FixRecurringError,
                    description: format!("Recurring error: {}. Occurred {} times.", error_type, count),
                    priority: 0.9,
                });
            }
        }
    }

    /// Get meta-cognitive insights
    pub fn get_insights(&self) -> MetaCognitiveInsights {
        let avg_confidence = if !self.confidence_history.is_empty() {
            self.confidence_history.iter().sum::<f32>() / self.confidence_history.len() as f32
        } else {
            0.0
        };

        let total_errors: usize = self.error_patterns.values().sum();

        MetaCognitiveInsights {
            total_steps: self.reasoning_trace.len(),
            average_confidence: avg_confidence,
            total_errors,
            unique_error_types: self.error_patterns.len(),
            suggestions_count: self.improvement_suggestions.len(),
        }
    }
}

impl Default for ReflectionModel {
    fn default() -> Self {
        Self::new()
    }
}

/// Individual reasoning step in the trace
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReasoningStep {
    pub id: Uuid,
    pub timestamp: String,
    pub step_type: StepType,
    pub input: String,
    pub output: String,
    pub confidence: f32,
    pub metadata: HashMap<String, String>,
}

impl ReasoningStep {
    pub fn new(step_type: StepType, input: String, output: String, confidence: f32) -> Self {
        Self {
            id: Uuid::new_v4(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            step_type,
            input,
            output,
            confidence,
            metadata: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum StepType {
    Query,
    Retrieval,
    Reasoning,
    Validation,
    Synthesis,
}

/// Improvement suggestion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Suggestion {
    pub id: Uuid,
    pub suggestion_type: SuggestionType,
    pub description: String,
    pub priority: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SuggestionType {
    IncreaseConfidence,
    FixRecurringError,
    OptimizeRetrieval,
    ImproveReasoning,
    EnhanceValidation,
}

/// Meta-cognitive insights summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaCognitiveInsights {
    pub total_steps: usize,
    pub average_confidence: f32,
    pub total_errors: usize,
    pub unique_error_types: usize,
    pub suggestions_count: usize,
}
