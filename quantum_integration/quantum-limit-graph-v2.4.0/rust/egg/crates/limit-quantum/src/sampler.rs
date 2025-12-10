// crates/limit-quantum/src/sampler.rs
use serde::{Serialize, Deserialize};
use rand::Rng;

/// Quantum-inspired sampler for graph traversal
pub struct QuantumSampler {
    pub temperature: f32,
    pub num_samples: usize,
}

impl QuantumSampler {
    pub fn new(temperature: f32, num_samples: usize) -> Self {
        Self {
            temperature,
            num_samples,
        }
    }

    /// Sample from probability distribution using quantum-inspired approach
    pub fn sample(&self, probabilities: &[f32]) -> Vec<usize> {
        let mut rng = rand::thread_rng();
        let mut samples = Vec::new();

        for _ in 0..self.num_samples {
            let r: f32 = rng.gen();
            let mut cumsum = 0.0;

            for (idx, &prob) in probabilities.iter().enumerate() {
                cumsum += prob;
                if r <= cumsum {
                    samples.push(idx);
                    break;
                }
            }
        }

        samples
    }

    /// Apply quantum-inspired annealing to probabilities
    pub fn anneal(&self, probabilities: &[f32]) -> Vec<f32> {
        let sum: f32 = probabilities.iter().map(|&p| (p / self.temperature).exp()).sum();
        probabilities
            .iter()
            .map(|&p| (p / self.temperature).exp() / sum)
            .collect()
    }

    /// Quantum walk step
    pub fn quantum_walk_step(&self, current_state: &[f32], transition_matrix: &[Vec<f32>]) -> Vec<f32> {
        let n = current_state.len();
        let mut next_state = vec![0.0; n];

        for i in 0..n {
            for j in 0..n {
                next_state[i] += transition_matrix[j][i] * current_state[j];
            }
        }

        // Normalize
        let sum: f32 = next_state.iter().sum();
        if sum > 0.0 {
            next_state.iter_mut().for_each(|x| *x /= sum);
        }

        next_state
    }
}

/// Quantum-inspired sampling result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SamplingResult {
    pub samples: Vec<usize>,
    pub probabilities: Vec<f32>,
    pub entropy: f32,
}

impl SamplingResult {
    pub fn new(samples: Vec<usize>, probabilities: Vec<f32>) -> Self {
        let entropy = Self::compute_entropy(&probabilities);
        Self {
            samples,
            probabilities,
            entropy,
        }
    }

    fn compute_entropy(probabilities: &[f32]) -> f32 {
        probabilities
            .iter()
            .filter(|&&p| p > 0.0)
            .map(|&p| -p * p.log2())
            .sum()
    }
}
