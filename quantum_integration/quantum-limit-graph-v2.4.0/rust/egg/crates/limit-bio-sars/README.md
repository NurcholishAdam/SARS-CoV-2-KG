# LIMIT-BIO-SARS: SARS-CoV-2 Biomedical Knowledge Graph

Enriched biomedical knowledge graph for SARS-CoV-2 research with comprehensive node types, provenance tracking, and loader utilities.

## Features

### Enriched Node Types
- **VirusNode**: Virus information with taxonomy, host species, and provenance
- **ProteinNode**: Protein data with sequences, structures, and binding sites
- **HostReceptorNode**: Receptor information with tissue distribution and expression levels
- **VariantNode**: Variant tracking with mutations, lineage, and epidemiological data
- **TherapyNode**: Treatment information with clinical trial data and efficacy

### Graph Operations
- Build comprehensive biomedical graphs
- Link nodes with confidence scores and provenance
- Query nodes by name or properties
- Track edges with metadata and evidence

### Data Loading
- Load nodes from JSON files
- Load edges with provenance
- Load corpus documents from JSONL
- Track loading statistics

## Usage

```rust
use limit_bio_sars::{VirusNode, ProteinNode, BioGraph};

// Create virus node
let virus = VirusNode::new("SARS-CoV-2".to_string(), 29.9);
let mut graph = BioGraph::new(virus);

// Add protein
let spike = ProteinNode::new("Spike Protein".to_string());
graph.add_protein(spike);

// Link with provenance
graph.link_with_confidence(
    spike.id,
    receptor.id,
    "binds_to",
    Some("High affinity".to_string()),
    0.95,
    vec!["PubMed:12345".to_string()]
);
```

## Integration

Part of the LIMIT-GRAPH v2.4.1 ecosystem for quantum-inspired AI research agents.
