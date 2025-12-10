// tests/rd_tests.rs
#[cfg(test)]
mod tests {
    use limit_quantum::rd::{RDPoint, RDCurve, RDOptimizer};

    #[test]
    fn test_rd_point_creation() {
        let point = RDPoint::new(0.8, 0.2, 32, "simulator".to_string());
        assert_eq!(point.rate, 0.8);
        assert_eq!(point.distortion, 0.2);
        assert_eq!(point.batch_size, 32);
        assert_eq!(point.backend, "simulator");
    }

    #[test]
    fn test_rd_curve_optimal() {
        let mut curve = RDCurve::new();
        curve.add_point(RDPoint::new(0.5, 0.5, 16, "sim".to_string()));
        curve.add_point(RDPoint::new(0.8, 0.2, 32, "sim".to_string()));
        curve.add_point(RDPoint::new(0.6, 0.4, 24, "sim".to_string()));

        curve.compute_optimal();
        let optimal = curve.get_optimal().unwrap();
        
        // Point with rate 0.8 and distortion 0.2 should be optimal
        assert_eq!(optimal.rate, 0.8);
        assert_eq!(optimal.distortion, 0.2);
    }

    #[test]
    fn test_rd_optimizer() {
        let mut optimizer = RDOptimizer::new();
        
        let mut curve1 = RDCurve::new();
        curve1.add_point(RDPoint::new(0.7, 0.3, 32, "sim".to_string()));
        curve1.add_point(RDPoint::new(0.9, 0.1, 64, "sim".to_string()));
        
        let mut curve2 = RDCurve::new();
        curve2.add_point(RDPoint::new(0.6, 0.4, 16, "qpu".to_string()));
        curve2.add_point(RDPoint::new(0.8, 0.2, 32, "qpu".to_string()));
        
        optimizer.add_curve(curve1);
        optimizer.add_curve(curve2);
        optimizer.optimize_all();
        
        assert_eq!(optimizer.curves.len(), 2);
        assert!(optimizer.curves[0].get_optimal().is_some());
        assert!(optimizer.curves[1].get_optimal().is_some());
    }

    #[test]
    fn test_empty_curve() {
        let mut curve = RDCurve::new();
        curve.compute_optimal();
        assert!(curve.get_optimal().is_none());
    }

    #[test]
    fn test_single_point_curve() {
        let mut curve = RDCurve::new();
        curve.add_point(RDPoint::new(0.75, 0.25, 32, "sim".to_string()));
        curve.compute_optimal();
        
        let optimal = curve.get_optimal().unwrap();
        assert_eq!(optimal.rate, 0.75);
        assert_eq!(optimal.distortion, 0.25);
    }
}
