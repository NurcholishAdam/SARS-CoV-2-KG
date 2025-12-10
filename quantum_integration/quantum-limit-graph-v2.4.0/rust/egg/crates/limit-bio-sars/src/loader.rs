// crates/limit-bio-sars/src/loader.rs
use crate::nodes::*;
use anyhow::{Result, Context};
use serde_json;
use std::fs;
use std::path::Path;

/// Loader for biomedical graph nodes from various sources
pub struct BioGraphLoader {
    pub nodes_loaded: usize,
    pub edges_loaded: usize,
}

impl BioGraphLoader {
    pub fn new() -> Self {
        Self {
            nodes_loaded: 0,
            edges_loaded: 0,
        }
    }

    /// Load virus nodes from JSON file
    pub fn load_virus_nodes(&mut self, path: &Path) -> Result<Vec<VirusNode>> {
        let content = fs::read_to_string(path)
            .context("Failed to read virus nodes file")?;
        let nodes: Vec<VirusNode> = serde_json::from_str(&content)
            .context("Failed to parse virus nodes JSON")?;
        self.nodes_loaded += nodes.len();
        Ok(nodes)
    }

    /// Load protein nodes from JSON file
    pub fn load_protein_nodes(&mut self, path: &Path) -> Result<Vec<ProteinNode>> {
        let content = fs::read_to_string(path)
            .context("Failed to read protein nodes file")?;
        let nodes: Vec<ProteinNode> = serde_json::from_str(&content)
            .context("Failed to parse protein nodes JSON")?;
        self.nodes_loaded += nodes.len();
        Ok(nodes)
    }

    /// Load receptor nodes from JSON file
    pub fn load_receptor_nodes(&mut self, path: &Path) -> Result<Vec<HostReceptorNode>> {
        let content = fs::read_to_string(path)
            .context("Failed to read receptor nodes file")?;
        let nodes: Vec<HostReceptorNode> = serde_json::from_str(&content)
            .context("Failed to parse receptor nodes JSON")?;
        self.nodes_loaded += nodes.len();
        Ok(nodes)
    }

    /// Load variant nodes from JSON file
    pub fn load_variant_nodes(&mut self, path: &Path) -> Result<Vec<VariantNode>> {
        let content = fs::read_to_string(path)
            .context("Failed to read variant nodes file")?;
        let nodes: Vec<VariantNode> = serde_json::from_str(&content)
            .context("Failed to parse variant nodes JSON")?;
        self.nodes_loaded += nodes.len();
        Ok(nodes)
    }

    /// Load therapy nodes from JSON file
    pub fn load_therapy_nodes(&mut self, path: &Path) -> Result<Vec<TherapyNode>> {
        let content = fs::read_to_string(path)
            .context("Failed to read therapy nodes file")?;
        let nodes: Vec<TherapyNode> = serde_json::from_str(&content)
            .context("Failed to parse therapy nodes JSON")?;
        self.nodes_loaded += nodes.len();
        Ok(nodes)
    }

    /// Load edges from JSON file
    pub fn load_edges(&mut self, path: &Path) -> Result<Vec<Edge>> {
        let content = fs::read_to_string(path)
            .context("Failed to read edges file")?;
        let edges: Vec<Edge> = serde_json::from_str(&content)
            .context("Failed to parse edges JSON")?;
        self.edges_loaded += edges.len();
        Ok(edges)
    }

    /// Load corpus documents from JSONL file
    pub fn load_corpus(&self, path: &Path) -> Result<Vec<BioCorpusDoc>> {
        let content = fs::read_to_string(path)
            .context("Failed to read corpus file")?;
        let docs: Vec<BioCorpusDoc> = content
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(|line| serde_json::from_str(line))
            .collect::<Result<Vec<_>, _>>()
            .context("Failed to parse corpus JSONL")?;
        Ok(docs)
    }

    /// Get loading statistics
    pub fn stats(&self) -> LoaderStats {
        LoaderStats {
            nodes_loaded: self.nodes_loaded,
            edges_loaded: self.edges_loaded,
        }
    }
}

#[derive(Debug, Clone)]
pub struct LoaderStats {
    pub nodes_loaded: usize,
    pub edges_loaded: usize,
}

impl Default for BioGraphLoader {
    fn default() -> Self {
        Self::new()
    }
}
