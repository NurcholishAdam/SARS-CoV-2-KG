// crates/limit-bio-sars/src/nodes.rs
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use std::collections::HashMap;

/// Enriched virus node with metadata and provenance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirusNode {
    pub id: Uuid,
    pub name: String,
    pub genome_kb: f32,
    pub taxonomy: Option<String>,
    pub host_species: Vec<String>,
    pub metadata: HashMap<String, String>,
    pub provenance: Vec<String>, // DOI/source references
}

/// Enriched protein node with structural and functional data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProteinNode {
    pub id: Uuid,
    pub name: String,
    pub role: Option<String>,
    pub sequence: Option<String>,
    pub structure_pdb: Option<String>,
    pub binding_sites: Vec<String>,
    pub metadata: HashMap<String, String>,
}

/// Enriched host receptor node with tissue distribution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HostReceptorNode {
    pub id: Uuid,
    pub name: String,
    pub tissue: Option<String>,
    pub expression_level: Option<f32>,
    pub cell_types: Vec<String>,
    pub metadata: HashMap<String, String>,
}

/// Enriched variant node with epidemiological data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariantNode {
    pub id: Uuid,
    pub name: String,
    pub mutations: Vec<String>,
    pub lineage: Option<String>,
    pub first_detected: Option<String>,
    pub transmissibility: Option<f32>,
    pub immune_escape: Option<f32>,
    pub metadata: HashMap<String, String>,
}

/// Enriched therapy node with clinical trial data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TherapyNode {
    pub id: Uuid,
    pub name: String,
    pub mechanism: String,
    pub trial_phase: Option<String>,
    pub efficacy: Option<f32>,
    pub side_effects: Vec<String>,
    pub approval_status: Option<String>,
    pub metadata: HashMap<String, String>,
}

/// Enriched edge with provenance and confidence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Edge {
    pub src: Uuid,
    pub dst: Uuid,
    pub relation: String,     // binds_to, neutralizes, treats, expressed_in
    pub evidence: Option<String>,
    pub confidence: f32,
    pub provenance: Vec<String>,
    pub metadata: HashMap<String, String>,
}

/// Biomedical corpus document with enriched metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BioCorpusDoc {
    pub source: String,       // DOI/URL
    pub text: String,
    pub domain: String,       // Virology/Genomics/Treatment/etc.
    pub authors: Vec<String>,
    pub publication_date: Option<String>,
    pub citations: Vec<String>,
    pub keywords: Vec<String>,
}

impl VirusNode {
    pub fn new(name: String, genome_kb: f32) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            genome_kb,
            taxonomy: None,
            host_species: vec![],
            metadata: HashMap::new(),
            provenance: vec![],
        }
    }
}

impl ProteinNode {
    pub fn new(name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            role: None,
            sequence: None,
            structure_pdb: None,
            binding_sites: vec![],
            metadata: HashMap::new(),
        }
    }
}

impl HostReceptorNode {
    pub fn new(name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            tissue: None,
            expression_level: None,
            cell_types: vec![],
            metadata: HashMap::new(),
        }
    }
}

impl VariantNode {
    pub fn new(name: String, mutations: Vec<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            mutations,
            lineage: None,
            first_detected: None,
            transmissibility: None,
            immune_escape: None,
            metadata: HashMap::new(),
        }
    }
}

impl TherapyNode {
    pub fn new(name: String, mechanism: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            mechanism,
            trial_phase: None,
            efficacy: None,
            side_effects: vec![],
            approval_status: None,
            metadata: HashMap::new(),
        }
    }
}
