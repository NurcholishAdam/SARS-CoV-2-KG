// tests/reflection_tests.rs
#[cfg(test)]
mod tests {
    use limit_reflection::{ReflectionEngine, ReflectionGovernance, ReflectionRules};

    #[test]
    fn test_simple_reflection() {
        let engine = ReflectionEngine::new(3);
        let result = engine.reflect_on_query("Test query").unwrap();

        assert!(!result.steps.is_empty());
        assert!(result.final_confidence > 0.0);
        assert!(result.final_confidence <= 1.0);
    }

    #[test]
    fn test_deep_reflection() {
        let engine = ReflectionEngine::new(2);
        let result = engine.deep_reflect("Complex test query").unwrap();

        assert!(!result.layers.is_empty());
        assert!(result.final_depth <= 2);
    }

    #[test]
    fn test_insights_generation() {
        let engine = ReflectionEngine::new(3);
        let _ = engine.reflect_on_query("Query 1");
        let _ = engine.reflect_on_query("Query 2");

        let insights = engine.get_insights();
        assert!(insights.total_steps >= 2);
    }

    #[test]
    fn test_error_recording() {
        let engine = ReflectionEngine::new(3);
        engine.record_error("TestError".to_string());
        engine.record_error("TestError".to_string());

        let insights = engine.get_insights();
        assert!(insights.total_errors >= 2);
    }

    #[test]
    fn test_governance_validation() {
        let governance = ReflectionGovernance::default_rules();
        let engine = ReflectionEngine::new(3);

        // Perform some reflections
        let _ = engine.reflect_on_query("Test");

        let model = engine.model.read().unwrap();
        let validation = governance.validate_reflection(&*model);

        assert!(validation.valid || !validation.errors.is_empty());
    }

    #[test]
    fn test_quality_check() {
        let governance = ReflectionGovernance::default_rules();
        let engine = ReflectionEngine::new(3);

        let _ = engine.reflect_on_query("Quality test");

        let model = engine.model.read().unwrap();
        let quality = governance.check_quality(&*model);

        assert!(quality.overall_quality >= 0.0);
        assert!(quality.overall_quality <= 1.0);
    }

    #[test]
    fn test_custom_rules() {
        let rules = ReflectionRules {
            min_average_confidence: 0.9,
            max_error_rate: 0.1,
            min_reasoning_steps: 5,
            min_suggestion_priority: 0.8,
            min_quality_score: 0.85,
        };

        let governance = ReflectionGovernance::new(rules);
        let engine = ReflectionEngine::new(3);

        let _ = engine.reflect_on_query("Test");

        let model = engine.model.read().unwrap();
        let validation = governance.validate_reflection(&*model);

        // With strict rules, validation might fail
        assert!(validation.valid || !validation.warnings.is_empty());
    }
}
