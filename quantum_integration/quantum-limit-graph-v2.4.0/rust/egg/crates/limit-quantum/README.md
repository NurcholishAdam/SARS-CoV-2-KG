# LIMIT-QUANTUM: Quantum-Inspired Retrieval and Optimization

Quantum-inspired algorithms for knowledge graph retrieval, rate-distortion optimization, and sampling.

## Features

### Rate-Distortion (RD) Optimization
- RD point tracking with rate, distortion, batch size, and backend
- RD curve construction and optimization
- Automatic optimal point selection
- Multi-curve optimization

### Quantum-Inspired Sampling
- Probability distribution sampling
- Quantum annealing for probability adjustment
- Quantum walk simulation
- Entropy computation

## Usage

### RD Optimization

```rust
use limit_quantum::{RDPoint, RDCurve};

let mut curve = RDCurve::new();
curve.add_point(RDPoint::new(0.8, 0.2, 32, "simulator".to_string()));
curve.add_point(RDPoint::new(0.9, 0.15, 64, "qpu".to_string()));

curve.compute_optimal();
if let Some(optimal) = curve.get_optimal() {
    println!("Optimal: rate={}, distortion={}", optimal.rate, optimal.distortion);
}
```

### Quantum Sampling

```rust
use limit_quantum::QuantumSampler;

let sampler = QuantumSampler::new(0.5, 100);
let probabilities = vec![0.3, 0.5, 0.2];
let samples = sampler.sample(&probabilities);
let annealed = sampler.anneal(&probabilities);
```

## Integration

Optimizes retrieval performance for SARS-CoV-2 knowledge graph queries.
