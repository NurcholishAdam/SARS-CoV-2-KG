// crates/limit-hub/src/main2.rs
// Combined Hub + Reflection server

use tracing_subscriber;
use limit_bio_sars::BioGraph;

mod api2;
mod governance;
mod state;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    tracing::info!("Initializing LIMIT Hub with Reflection...");

    // Initialize biomedical graph (optional)
    let bio_graph = initialize_graph();

    // Create combined router
    let app = api2::create_combined_router();

    // Start server on port 3002
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3002")
        .await
        .unwrap();

    tracing::info!(
        "LIMIT Hub + Reflection API listening on {}",
        listener.local_addr().unwrap()
    );
    tracing::info!("Endpoints:");
    tracing::info!("  Hub: /submit, /submissions, /validate");
    tracing::info!("  Reflection: /reflect, /deep-reflect, /insights");
    tracing::info!("  Combined: /reflect-with-evidence");

    axum::serve(listener, app).await.unwrap();
}

/// Initialize biomedical graph with sample data
fn initialize_graph() -> Option<BioGraph> {
    tracing::info!("Initializing biomedical graph...");
    
    match BioGraph::new() {
        Ok(mut graph) => {
            // Add sample SARS-CoV-2 nodes
            let spike_id = graph.add_protein_node(
                "Spike Protein".to_string(),
                "SARS-CoV-2 spike glycoprotein".to_string(),
            );
            
            let ace2_id = graph.add_protein_node(
                "ACE2 Receptor".to_string(),
                "Angiotensin-converting enzyme 2".to_string(),
            );
            
            // Add relationship
            graph.add_edge(spike_id, ace2_id, "binds_to".to_string());
            
            tracing::info!("Graph initialized with {} nodes", graph.node_count());
            Some(graph)
        }
        Err(e) => {
            tracing::warn!("Failed to initialize graph: {}", e);
            None
        }
    }
}
