# SARS-CoV-2 Extended Implementation - Quick Start

## Overview

This guide helps you get started with the extended SARS-CoV-2 knowledge graph implementation featuring:
- Enriched biomedical graph with loader
- Multi-intent harness with provenance
- Quantum-inspired retrieval optimization
- Open-source hub with governance
- Comprehensive unit tests

## Prerequisites

- Rust 1.70+ installed
- Cargo package manager
- Basic understanding of knowledge graphs

## Installation

```bash
cd quantum_integration/quantum-limit-graph-v2.4.0/rust/egg
cargo build --release
```

## Quick Demo

Run the complete demo showcasing all 5 stages:

```bash
cargo run --example sarscov2_complete_demo
```

Expected output:
```
=== SARS-CoV-2 Knowledge Graph Complete Demo ===

Stage 1: Building enriched SARS-CoV-2 graph...
  Nodes: 6, Edges: 1

Stage 2: Running multi-intent benchmark...
  Queries: 1, Avg Coverage: 0.85

Stage 3: Quantum-inspired retrieval optimization...
  Optimal: Rate=0.90, Distortion=0.15, Batch=64

Stage 4: Validating submission with governance...
  Valid: true, Errors: 0, Warnings: 0

=== Demo Complete ===
```

## Running Tests

### All Tests
```bash
cargo test
```

### Specific Crate Tests
```bash
# RD optimization tests
cargo test -p limit-quantum

# Governance tests
cargo test -p limit-hub

# Benchmark tests
cargo test -p limit-benchmark
```

## Starting the Hub API

```bash
cargo run --bin limit-hub
```

The API will be available at `http://localhost:3000`

### Test API Endpoints

```bash
# Health check
curl http://localhost:3000/health

# Submit data
curl -X POST http://localhost:3000/submit \
  -H "Content-Type: application/json" \
  -d '{
    "id": "sub-001",
    "content": "New variant data",
    "confidence": 0.85,
    "provenance": ["PubMed", "bioRxiv"],
    "quality_score": 0.9,
    "metadata": {}
  }'

# List submissions
curl http://localhost:3000/submissions
```

## Stage-by-Stage Usage

### Stage 1: Build Enriched Graph

```rust
use limit_bio_sars::{VirusNode, ProteinNode, BioGraph};

let virus = VirusNode::new("SARS-CoV-2".to_string(), 29.9);
let mut graph = BioGraph::new(virus);

let spike = ProteinNode::new("Spike Protein".to_string());
graph.add_protein(spike.clone());

graph.link_with_confidence(
    spike.id,
    receptor.id,
    "binds_to",
    Some("High affinity".to_string()),
    0.95,
    vec!["PubMed:12345".to_string()]
);
```

### Stage 2: Multi-Intent Benchmark

```rust
use limit_benchmark::{MultiIntentQuery, Intent, IntentType, MultiIntentHarness};

let query = MultiIntentQuery {
    id: "q1".to_string(),
    intents: vec![
        Intent {
            intent_type: IntentType::Factual,
            query: "What is spike protein?".to_string(),
            priority: 1.0,
            domain: Some("Virology".to_string()),
        },
    ],
    context: HashMap::new(),
};

let mut harness = MultiIntentHarness::new();
harness.add_query(query);
let results = harness.run_benchmark(executor);
```

### Stage 3: Quantum-Inspired Retrieval

```rust
use limit_quantum::{RDPoint, RDCurve, QuantumSampler};

let mut curve = RDCurve::new();
curve.add_point(RDPoint::new(0.8, 0.2, 32, "simulator".to_string()));
curve.compute_optimal();

let sampler = QuantumSampler::new(0.5, 100);
let samples = sampler.sample(&probabilities);
```

### Stage 4: Governance Validation

```rust
use limit_hub::governance::{GovernanceRules, Submission};

let rules = GovernanceRules::default_rules();
let submission = Submission { /* ... */ };
let validation = rules.validate_submission(&submission);
```

### Stage 5: Run Tests

```bash
cargo test --test rd_tests
cargo test --test governance_tests
```

## Project Structure

```
crates/
├── limit-bio-sars/       # Stage 1: Enriched graph
│   ├── src/
│   │   ├── nodes.rs      # Enriched node types
│   │   ├── graph.rs      # Graph operations
│   │   └── loader.rs     # Data loading
│   └── Cargo.toml
├── limit-benchmark/      # Stage 2: Multi-intent
│   ├── src/
│   │   ├── multi.rs      # Multi-intent queries
│   │   ├── metrics.rs    # Performance metrics
│   │   ├── harness.rs    # Benchmark execution
│   │   └── provenance.rs # Tracking
│   └── Cargo.toml
├── limit-quantum/        # Stage 3: Quantum-inspired
│   ├── src/
│   │   ├── rd.rs         # Rate-distortion
│   │   └── sampler.rs    # Quantum sampling
│   ├── tests/
│   │   └── rd_tests.rs   # Stage 5: Tests
│   └── Cargo.toml
└── limit-hub/            # Stage 4: Open-source hub
    ├── src/
    │   ├── governance.rs # Governance rules
    │   ├── api.rs        # REST API
    │   └── main.rs       # Server
    └── Cargo.toml
```

## Next Steps

1. **Extend the graph**: Add more node types and relationships
2. **Load real data**: Use the loader to import actual SARS-CoV-2 data
3. **Customize governance**: Adjust rules for your use case
4. **Deploy the hub**: Set up production environment
5. **Integrate with Python**: Use PyO3 for Python bindings

## Troubleshooting

### Build Errors
```bash
# Clean and rebuild
cargo clean
cargo build --release
```

### Test Failures
```bash
# Run tests with output
cargo test -- --nocapture
```

### API Connection Issues
```bash
# Check if port 3000 is available
netstat -an | grep 3000
```

## Documentation

- [Extended Delivery Document](./SARSCOV2_EXTENDED_DELIVERY.md)
- [Architecture Overview](./SARSCOV2_ARCHITECTURE.md)
- Individual crate READMEs in each crate directory

## Support

For issues or questions, refer to the main project documentation or create an issue in the repository.
