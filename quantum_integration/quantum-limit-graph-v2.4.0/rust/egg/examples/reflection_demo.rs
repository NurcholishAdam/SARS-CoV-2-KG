// examples/reflection_demo.rs
use limit_reflection::{ReflectionEngine, ReflectionGovernance};

fn main() {
    println!("=== LIMIT Reflection: Meta-Cognitive Reasoning Demo ===\n");

    // Create reflection engine
    let engine = ReflectionEngine::new(3);
    let governance = ReflectionGovernance::default_rules();

    // Example 1: Simple reflection
    println!("Example 1: Simple Reflection");
    let query1 = "What are the binding mechanisms of SARS-CoV-2 spike protein to ACE2 receptor?";
    match engine.reflect_on_query(query1) {
        Ok(result) => {
            println!("  Query: {}", query1);
            println!("  Steps: {}", result.steps.len());
            println!("  Confidence: {:.2}", result.final_confidence);
            println!("  Insights: {:?}\n", result.insights);
        }
        Err(e) => println!("  Error: {}\n", e),
    }

    // Example 2: Deep reflection
    println!("Example 2: Deep Reflection (Multi-Layer)");
    let query2 = "Analyze the effectiveness of mRNA vaccines against variants";
    match engine.deep_reflect(query2) {
        Ok(result) => {
            println!("  Query: {}", query2);
            println!("  Reflection Layers: {}", result.layers.len());
            println!("  Final Depth: {}", result.final_depth);
            if let Some(last_layer) = result.layers.last() {
                println!("  Final Confidence: {:.2}\n", last_layer.final_confidence);
            }
        }
        Err(e) => println!("  Error: {}\n", e),
    }

    // Example 3: Get insights
    println!("Example 3: Meta-Cognitive Insights");
    let insights = engine.get_insights();
    println!("  Total Steps: {}", insights.total_steps);
    println!("  Average Confidence: {:.2}", insights.average_confidence);
    println!("  Total Errors: {}", insights.total_errors);
    println!("  Suggestions: {}\n", insights.suggestions_count);

    // Example 4: Get improvement suggestions
    println!("Example 4: Improvement Suggestions");
    let suggestions = engine.get_suggestions();
    println!("  Total Suggestions: {}", suggestions.len());
    for (i, suggestion) in suggestions.iter().enumerate() {
        println!("  {}. {:?} (priority: {:.2})", i + 1, suggestion.suggestion_type, suggestion.priority);
        println!("     {}", suggestion.description);
    }
    println!();

    // Example 5: Quality check
    println!("Example 5: Quality Governance");
    let model = engine.model.read().unwrap();
    let quality = governance.check_quality(&*model);
    println!("  Overall Quality: {:.2}", quality.overall_quality);
    println!("  Confidence Score: {:.2}", quality.confidence_score);
    println!("  Error Score: {:.2}", quality.error_score);
    println!("  Completeness Score: {:.2}", quality.completeness_score);
    println!("  Meets Standards: {}\n", quality.meets_standards);

    println!("=== Demo Complete ===");
}
