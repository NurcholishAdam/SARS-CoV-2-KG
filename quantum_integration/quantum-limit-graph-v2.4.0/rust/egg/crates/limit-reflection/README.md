# LIMIT-REFLECTION: Meta-Cognitive Reasoning Engine

Meta-cognitive reasoning engine for SARS-CoV-2 knowledge graph with self-reflection, quantum-inspired validation, and governance.

## Features

### Meta-Cognitive Reasoning
- Self-reflective reasoning traces
- Confidence tracking and analysis
- Error pattern recognition
- Automatic improvement suggestions

### Quantum-Inspired Reflection
- Quantum sampling for validation
- Probability-based confidence computation
- Quantum annealing for optimization
- Entropy-based uncertainty measurement

### Governance & Quality Control
- Reflection validation rules
- Quality scoring system
- Suggestion approval workflow
- Standards compliance checking

## Architecture

```
┌─────────────────────────────────────────┐
│        Meta-Cognitive Reasoning         │
├─────────────────────────────────────────┤
│                                         │
│  model.rs                               │
│  ├─ ReflectionModel                     │
│  ├─ ReasoningStep                       │
│  └─ Suggestion Generation               │
│                                         │
│  engine.rs                              │
│  ├─ ReflectionEngine                    │
│  ├─ Complexity Analysis                 │
│  ├─ Meta-Reasoning                      │
│  └─ Deep Reflection (Recursive)         │
│                                         │
│  quantum.rs                             │
│  ├─ QuantumReflector                    │
│  ├─ Quantum Sampling                    │
│  └─ Confidence Computation              │
│                                         │
│  govern.rs                              │
│  ├─ ReflectionGovernance                │
│  ├─ Validation Rules                    │
│  └─ Quality Reports                     │
│                                         │
│  api.rs                                 │
│  └─ REST API Endpoints                  │
│                                         │
└─────────────────────────────────────────┘
```

## Usage

### Basic Reflection

```rust
use limit_reflection::ReflectionEngine;

let engine = ReflectionEngine::new(3);
let result = engine.reflect_on_query("What is spike protein?")?;

println!("Steps: {}", result.steps.len());
println!("Confidence: {:.2}", result.final_confidence);
```

### Deep Reflection

```rust
let result = engine.deep_reflect("Complex biomedical query")?;

println!("Layers: {}", result.layers.len());
println!("Final Depth: {}", result.final_depth);
```

### Get Insights

```rust
let insights = engine.get_insights();

println!("Total Steps: {}", insights.total_steps);
println!("Avg Confidence: {:.2}", insights.average_confidence);
println!("Errors: {}", insights.total_errors);
```

### Governance

```rust
use limit_reflection::ReflectionGovernance;

let governance = ReflectionGovernance::default_rules();
let model = engine.model.read().unwrap();

let validation = governance.validate_reflection(&*model);
let quality = governance.check_quality(&*model);

println!("Valid: {}", validation.valid);
println!("Quality: {:.2}", quality.overall_quality);
```

## API Endpoints

### Start Server

```bash
cargo run --bin limit-reflection
```

Server runs on `http://0.0.0.0:3001`

### Endpoints

- `GET /health` - Health check
- `POST /reflect` - Perform reflection on query
- `POST /deep-reflect` - Perform deep multi-layer reflection
- `GET /insights` - Get meta-cognitive insights
- `GET /suggestions` - Get improvement suggestions
- `GET /quality` - Check quality report

### Example API Call

```bash
curl -X POST http://localhost:3001/reflect \
  -H "Content-Type: application/json" \
  -d '{"query": "What is ACE2 receptor?"}'
```

## Integration with SARS-CoV-2 System

```rust
use limit_bio_sars::BioGraph;
use limit_reflection::ReflectionEngine;

// Build graph
let graph = build_sarscov2_graph();

// Reflect on graph operations
let engine = ReflectionEngine::new(3);
let result = engine.reflect_on_query("Analyze spike-ACE2 binding")?;

// Use insights to improve queries
let insights = engine.get_insights();
if insights.average_confidence < 0.7 {
    // Adjust query strategy
}
```

## Key Concepts

### Meta-Cognitive Reasoning
The engine monitors its own reasoning process, tracking:
- Reasoning steps and their confidence
- Error patterns and frequencies
- Improvement opportunities
- Quality metrics

### Quantum-Inspired Validation
Uses quantum-inspired techniques:
- Probability distributions from input features
- Quantum sampling for state exploration
- Entropy-based confidence measurement
- Annealing for optimization

### Governance
Ensures quality through:
- Minimum confidence thresholds
- Maximum error rates
- Reasoning depth requirements
- Suggestion priority filtering

## Testing

```bash
cargo test -p limit-reflection
```

## Performance

- Reflection: ~10ms per query
- Deep Reflection: ~30ms for 3 layers
- API Latency: <20ms per request
- Memory: ~50MB for 1000 reflections

## Dependencies

- limit-bio-sars: Biomedical graph integration
- limit-benchmark: Performance metrics
- limit-quantum: Quantum-inspired algorithms
- limit-hub: Governance framework

## Version

**v2.4.1** - Meta-Cognitive Reasoning Extension

Part of LIMIT-GRAPH SARS-CoV-2 Extended Implementation
