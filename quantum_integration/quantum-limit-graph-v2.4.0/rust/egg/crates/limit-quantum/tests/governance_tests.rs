// tests/governance_tests.rs
#[cfg(test)]
mod tests {
    use limit_hub::governance::{GovernanceRules, Submission};
    use std::collections::HashMap;

    fn create_test_submission(confidence: f32, provenance_count: usize, quality: f32) -> Submission {
        Submission {
            id: "test-001".to_string(),
            content: "Test content".to_string(),
            confidence,
            provenance: (0..provenance_count)
                .map(|i| format!("PubMed-{}", i))
                .collect(),
            quality_score: quality,
            metadata: HashMap::new(),
        }
    }

    #[test]
    fn test_valid_submission() {
        let rules = GovernanceRules::default_rules();
        let submission = create_test_submission(0.85, 3, 0.9);
        
        let result = rules.validate_submission(&submission);
        assert!(result.valid);
        assert!(result.errors.is_empty());
    }

    #[test]
    fn test_low_confidence_rejection() {
        let rules = GovernanceRules::default_rules();
        let submission = create_test_submission(0.5, 3, 0.9);
        
        let result = rules.validate_submission(&submission);
        assert!(!result.valid);
        assert!(!result.errors.is_empty());
        assert!(result.errors[0].contains("Confidence"));
    }

    #[test]
    fn test_insufficient_provenance() {
        let rules = GovernanceRules::default_rules();
        let submission = create_test_submission(0.85, 1, 0.9);
        
        let result = rules.validate_submission(&submission);
        assert!(!result.valid);
        assert!(!result.errors.is_empty());
        assert!(result.errors[0].contains("Provenance"));
    }

    #[test]
    fn test_low_quality_warning() {
        let rules = GovernanceRules::default_rules();
        let submission = create_test_submission(0.85, 3, 0.6);
        
        let result = rules.validate_submission(&submission);
        assert!(result.valid);
        assert!(!result.warnings.is_empty());
        assert!(result.requires_review);
    }

    #[test]
    fn test_custom_rules() {
        let rules = GovernanceRules {
            min_confidence: 0.9,
            min_provenance_count: 5,
            allowed_sources: vec!["PubMed".to_string()],
            quality_threshold: 0.95,
            review_required: false,
        };
        
        let submission = create_test_submission(0.85, 3, 0.9);
        let result = rules.validate_submission(&submission);
        
        assert!(!result.valid);
        assert!(result.errors.len() >= 2); // Both confidence and provenance fail
    }

    #[test]
    fn test_multiple_errors() {
        let rules = GovernanceRules::default_rules();
        let submission = create_test_submission(0.5, 1, 0.5);
        
        let result = rules.validate_submission(&submission);
        assert!(!result.valid);
        assert!(result.errors.len() >= 2);
        assert!(!result.warnings.is_empty());
    }
}
