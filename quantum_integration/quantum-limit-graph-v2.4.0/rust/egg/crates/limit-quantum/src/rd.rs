// crates/limit-quantum/src/rd.rs
use serde::{Serialize, Deserialize};

/// Rate-Distortion (RD) point for quantum-inspired retrieval
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RDPoint {
    pub rate: f32,
    pub distortion: f32,
    pub batch_size: usize,
    pub backend: String,
}

impl RDPoint {
    pub fn new(rate: f32, distortion: f32, batch_size: usize, backend: String) -> Self {
        Self {
            rate,
            distortion,
            batch_size,
            backend,
        }
    }
}

/// RD curve for optimization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RDCurve {
    pub points: Vec<RDPoint>,
    pub optimal_point: Option<RDPoint>,
}

impl RDCurve {
    pub fn new() -> Self {
        Self {
            points: vec![],
            optimal_point: None,
        }
    }

    pub fn add_point(&mut self, point: RDPoint) {
        self.points.push(point);
    }

    pub fn compute_optimal(&mut self) {
        if self.points.is_empty() {
            return;
        }

        // Find point with best rate-distortion tradeoff
        let optimal = self
            .points
            .iter()
            .min_by(|a, b| {
                let score_a = a.rate / (a.distortion + 1e-6);
                let score_b = b.rate / (b.distortion + 1e-6);
                score_a.partial_cmp(&score_b).unwrap()
            })
            .cloned();

        self.optimal_point = optimal;
    }

    pub fn get_optimal(&self) -> Option<&RDPoint> {
        self.optimal_point.as_ref()
    }
}

impl Default for RDCurve {
    fn default() -> Self {
        Self::new()
    }
}

/// RD optimizer for quantum-inspired retrieval
pub struct RDOptimizer {
    pub curves: Vec<RDCurve>,
}

impl RDOptimizer {
    pub fn new() -> Self {
        Self { curves: vec![] }
    }

    pub fn add_curve(&mut self, curve: RDCurve) {
        self.curves.push(curve);
    }

    pub fn optimize_all(&mut self) {
        for curve in &mut self.curves {
            curve.compute_optimal();
        }
    }
}

impl Default for RDOptimizer {
    fn default() -> Self {
        Self::new()
    }
}
