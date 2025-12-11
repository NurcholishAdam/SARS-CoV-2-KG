// crates/limit-reflection/src/lib.rs
pub mod api;
pub mod engine;
pub mod govern;
pub mod model;
pub mod quantum;

pub use api::{create_router, ReflectionApiState};
pub use engine::{ReflectionEngine, ReflectionResult, DeepReflectionResult};
pub use govern::{ReflectionGovernance, ReflectionRules, ReflectionValidation, QualityReport};
pub use model::{ReflectionModel, ReasoningStep, StepType, Suggestion, SuggestionType, MetaCognitiveInsights};
pub use quantum::QuantumReflector;
