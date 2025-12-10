# LIMIT-BENCHMARK: Multi-Intent Benchmarking Framework

Comprehensive benchmarking framework for SARS-CoV-2 knowledge graph with multi-intent queries, provenance tracking, and performance metrics.

## Features

### Multi-Intent Queries
- Support for multiple intent types: Factual, Causal, Comparative, Predictive, Exploratory
- Priority-based intent handling
- Domain-specific query routing
- Context-aware query execution

### Benchmark Harness
- Automated benchmark execution
- Latency and throughput measurement
- Intent coverage tracking
- Success rate monitoring

### Metrics
- Graph metrics: node count, edge count, density, coverage
- Query metrics: latency, throughput, accuracy, precision, recall
- F1 score computation
- Provenance scoring

### Provenance Tracking
- Record all operations with timestamps
- Track confidence scores
- Filter by source or confidence
- Comprehensive audit trail

## Usage

```rust
use limit_benchmark::{MultiIntentQuery, Intent, IntentType, MultiIntentHarness};

// Create multi-intent query
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

// Run benchmark
let mut harness = MultiIntentHarness::new();
harness.add_query(query);
let results = harness.run_benchmark(executor);
```

## Integration

Works seamlessly with limit-bio-sars for SARS-CoV-2 knowledge graph benchmarking.
