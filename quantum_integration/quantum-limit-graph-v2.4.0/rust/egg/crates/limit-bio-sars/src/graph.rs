// crates/limit-bio-sars/src/graph.rs
use crate::nodes::*;
use uuid::Uuid;
use std::collections::HashMap;

/// Enriched biomedical knowledge graph for SARS-CoV-2
#[derive(Debug, Clone)]
pub struct BioGraph {
    pub id: Uuid,
    pub virus: VirusNode,
    pub proteins: Vec<ProteinNode>,
    pub receptors: Vec<HostReceptorNode>,
    pub variants: Vec<VariantNode>,
    pub therapies: Vec<TherapyNode>,
    pub edges: Vec<Edge>,
    pub metadata: HashMap<String, String>,
}

impl BioGraph {
    pub fn new(virus: VirusNode) -> Self {
        Self {
            id: Uuid::new_v4(),
            virus,
            proteins: vec![],
            receptors: vec![],
            variants: vec![],
            therapies: vec![],
            edges: vec![],
            metadata: HashMap::new(),
        }
    }

    pub fn add_protein(&mut self, p: ProteinNode) {
        self.proteins.push(p);
    }

    pub fn add_receptor(&mut self, r: HostReceptorNode) {
        self.receptors.push(r);
    }

    pub fn add_variant(&mut self, v: VariantNode) {
        self.variants.push(v);
    }

    pub fn add_therapy(&mut self, t: TherapyNode) {
        self.therapies.push(t);
    }

    pub fn link(&mut self, src: Uuid, dst: Uuid, relation: &str, evidence: Option<String>) {
        self.edges.push(Edge {
            src,
            dst,
            relation: relation.into(),
            evidence,
            confidence: 1.0,
            provenance: vec![],
            metadata: HashMap::new(),
        });
    }

    pub fn link_with_confidence(
        &mut self,
        src: Uuid,
        dst: Uuid,
        relation: &str,
        evidence: Option<String>,
        confidence: f32,
        provenance: Vec<String>,
    ) {
        self.edges.push(Edge {
            src,
            dst,
            relation: relation.into(),
            evidence,
            confidence,
            provenance,
            metadata: HashMap::new(),
        });
    }

    /// Find all edges connected to a node
    pub fn edges_for_node(&self, node_id: Uuid) -> Vec<&Edge> {
        self.edges
            .iter()
            .filter(|e| e.src == node_id || e.dst == node_id)
            .collect()
    }

    /// Get node count
    pub fn node_count(&self) -> usize {
        1 + self.proteins.len()
            + self.receptors.len()
            + self.variants.len()
            + self.therapies.len()
    }

    /// Get edge count
    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    /// Find protein by name
    pub fn find_protein(&self, name: &str) -> Option<&ProteinNode> {
        self.proteins.iter().find(|p| p.name == name)
    }

    /// Find variant by name
    pub fn find_variant(&self, name: &str) -> Option<&VariantNode> {
        self.variants.iter().find(|v| v.name == name)
    }

    /// Find therapy by name
    pub fn find_therapy(&self, name: &str) -> Option<&TherapyNode> {
        self.therapies.iter().find(|t| t.name == name)
    }
}
