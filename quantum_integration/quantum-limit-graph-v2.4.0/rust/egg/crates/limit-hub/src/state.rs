// crates/limit-hub/src/state.rs
use limit_bio_sars::BioGraph;
use limit_reflection::ReflectionEngine;
use crate::governance::{GovernanceRules, Submission};

/// Combined hub state with reflection and graph
pub struct CombinedHubState {
    pub governance: GovernanceRules,
    pub submissions: Vec<Submission>,
    pub reflection_engine: ReflectionEngine,
    pub bio_graph: Option<BioGraph>,
}

impl CombinedHubState {
    pub fn new() -> Self {
        Self {
            governance: GovernanceRules::default_rules(),
            submissions: vec![],
            reflection_engine: ReflectionEngine::new(3),
            bio_graph: None,
        }
    }

    /// Initialize with biomedical graph
    pub fn with_graph(mut self, graph: BioGraph) -> Self {
        self.bio_graph = Some(graph);
        self
    }

    /// Get reflection insights
    pub fn get_reflection_insights(&self) -> limit_reflection::MetaCognitiveInsights {
        self.reflection_engine.get_insights()
    }

    /// Reflect on a query with graph context
    pub fn reflect_with_context(&self, query: &str) -> anyhow::Result<limit_reflection::ReflectionResult> {
        // Add graph context if available
        let enriched_query = if let Some(ref graph) = self.bio_graph {
            format!("{} [Graph nodes: {}]", query, graph.node_count())
        } else {
            query.to_string()
        };

        self.reflection_engine.reflect_on_query(&enriched_query)
    }
}

impl Default for CombinedHubState {
    fn default() -> Self {
        Self::new()
    }
}
