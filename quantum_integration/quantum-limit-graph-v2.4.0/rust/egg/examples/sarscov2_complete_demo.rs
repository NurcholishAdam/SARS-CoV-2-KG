// examples/sarscov2_complete_demo.rs
use limit_bio_sars::{VirusNode, ProteinNode, HostReceptorNode, VariantNode, TherapyNode, BioGraph};
use limit_benchmark::{MultiIntentQuery, Intent, IntentType, MultiIntentHarness, MultiIntentResult};
use limit_quantum::{RDPoint, RDCurve, QuantumSampler};
use limit_hub::governance::{GovernanceRules, Submission};
use std::collections::HashMap;

fn main() {
    println!("=== SARS-CoV-2 Knowledge Graph Complete Demo ===\n");

    // Stage 1: Build enriched biomedical graph
    println!("Stage 1: Building enriched SARS-CoV-2 graph...");
    let mut graph = build_sarscov2_graph();
    println!("  Nodes: {}, Edges: {}\n", graph.node_count(), graph.edge_count());

    // Stage 2: Multi-intent harness with provenance
    println!("Stage 2: Running multi-intent benchmark...");
    let benchmark_results = run_multi_intent_benchmark();
    println!("  Queries: {}, Avg Coverage: {:.2}\n",
        benchmark_results.total_queries,
        benchmark_results.avg_intent_coverage
    );

    // Stage 3: Quantum-inspired retrieval and RD optimization
    println!("Stage 3: Quantum-inspired retrieval optimization...");
    let rd_curve = optimize_retrieval();
    if let Some(optimal) = rd_curve.get_optimal() {
        println!("  Optimal: Rate={:.2}, Distortion={:.2}, Batch={}\n",
            optimal.rate, optimal.distortion, optimal.batch_size
        );
    }

    // Stage 4: Open-source hub with governance
    println!("Stage 4: Validating submission with governance...");
    let validation = validate_with_governance();
    println!("  Valid: {}, Errors: {}, Warnings: {}\n",
        validation.valid,
        validation.errors.len(),
        validation.warnings.len()
    );

    println!("=== Demo Complete ===");
}

fn build_sarscov2_graph() -> BioGraph {
    let virus = VirusNode::new("SARS-CoV-2".to_string(), 29.9);
    let mut graph = BioGraph::new(virus);

    // Add proteins
    let spike = ProteinNode::new("Spike Protein".to_string());
    let ace2_receptor = HostReceptorNode::new("ACE2".to_string());
    
    graph.add_protein(spike.clone());
    graph.add_receptor(ace2_receptor.clone());

    // Add variants
    let delta = VariantNode::new(
        "Delta".to_string(),
        vec!["L452R".to_string(), "T478K".to_string()]
    );
    let omicron = VariantNode::new(
        "Omicron".to_string(),
        vec!["N501Y".to_string(), "E484A".to_string()]
    );
    
    graph.add_variant(delta);
    graph.add_variant(omicron);

    // Add therapies
    let vaccine = TherapyNode::new(
        "mRNA Vaccine".to_string(),
        "Induces neutralizing antibodies".to_string()
    );
    graph.add_therapy(vaccine);

    // Link nodes with provenance
    graph.link_with_confidence(
        spike.id,
        ace2_receptor.id,
        "binds_to",
        Some("High affinity binding".to_string()),
        0.95,
        vec!["PubMed:12345".to_string(), "Nature:2020".to_string()]
    );

    graph
}

fn run_multi_intent_benchmark() -> limit_benchmark::BenchmarkSummary {
    let mut harness = MultiIntentHarness::new();

    // Add multi-intent queries
    let query1 = MultiIntentQuery {
        id: "q1".to_string(),
        intents: vec![
            Intent {
                intent_type: IntentType::Factual,
                query: "What is the spike protein?".to_string(),
                priority: 1.0,
                domain: Some("Virology".to_string()),
            },
            Intent {
                intent_type: IntentType::Causal,
                query: "How does spike bind to ACE2?".to_string(),
                priority: 0.8,
                domain: Some("Molecular Biology".to_string()),
            },
        ],
        context: HashMap::new(),
    };

    harness.add_query(query1);

    // Run benchmark
    harness.run_benchmark(|query| {
        MultiIntentResult {
            query_id: query.id.clone(),
            success: true,
            latency_ms: 150.0,
            intent_coverage: 0.85,
            provenance: vec!["PubMed".to_string()],
        }
    })
}

fn optimize_retrieval() -> RDCurve {
    let mut curve = RDCurve::new();

    // Add RD points for different configurations
    curve.add_point(RDPoint::new(0.6, 0.4, 16, "simulator".to_string()));
    curve.add_point(RDPoint::new(0.8, 0.2, 32, "simulator".to_string()));
    curve.add_point(RDPoint::new(0.9, 0.15, 64, "qpu".to_string()));

    curve.compute_optimal();
    curve
}

fn validate_with_governance() -> limit_hub::governance::ValidationResult {
    let rules = GovernanceRules::default_rules();
    
    let submission = Submission {
        id: "sub-001".to_string(),
        content: "New SARS-CoV-2 variant data".to_string(),
        confidence: 0.85,
        provenance: vec!["PubMed".to_string(), "bioRxiv".to_string()],
        quality_score: 0.9,
        metadata: HashMap::new(),
    };

    rules.validate_submission(&submission)
}
