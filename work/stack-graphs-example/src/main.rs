use stack_graphs::graph::{StackGraph, NodeID};

fn main() {
    // Create a new stack graph
    let mut graph = StackGraph::new();
    
    // Create a file handle
    let file_id = graph.get_or_create_file("example.rs");
    
    // Create some symbols (prefixed with underscore as they're currently unused)
    let _a_symbol = graph.add_symbol("A");
    let _foo_symbol = graph.add_symbol("foo");
    
    // Create a scope node for the file
    let scope_id = NodeID::new_in_file(file_id, 1);
    let scope_node = graph.add_scope_node(scope_id, true);
    
    println!("Created stack graph with:");
    println!("  - File: example.rs");
    println!("  - Symbols: A, foo");
    println!("  - Scope node: {:?}", scope_node);
    
    // Print completion message
    println!("\nStack-Graphs Example Completed");
    println!("Note: This is a minimal setup demonstrating basic graph construction.");
}
