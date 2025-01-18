use insta::assert_snapshot;
use stack_graphs::graph::{NodeID, StackGraph};

#[test]
fn test_basic_graph_construction() {
    let mut graph = StackGraph::new();
    let file_id = graph.get_or_create_file("test.py");

    // Create symbols
    let _sys_symbol = graph.add_symbol("sys");
    let _path_symbol = graph.add_symbol("path");

    // Create a scope node
    let scope_id = NodeID::new_in_file(file_id, 1);
    let _scope_node = graph.add_scope_node(scope_id, true);

    assert_snapshot!(format_graph_info(&mut graph));
}

#[test]
fn test_multiple_symbols() {
    let mut graph = StackGraph::new();
    let file_id = graph.get_or_create_file("test.py");

    // Create multiple symbols
    let symbols = ["os", "sys", "json", "path"];
    for symbol_name in symbols.iter() {
        graph.add_symbol(symbol_name);
    }

    // Create a scope node
    let scope_id = NodeID::new_in_file(file_id, 1);
    let _scope_node = graph.add_scope_node(scope_id, true);

    assert_snapshot!(format_graph_info(&mut graph));
}

fn format_graph_info(graph: &mut StackGraph) -> String {
    let mut info = String::new();

    // Basic graph information
    info.push_str(&format!("Total nodes: {}\n", graph.iter_nodes().count()));

    // Display nodes
    info.push_str("\nNodes:\n");
    for node in graph.iter_nodes() {
        info.push_str(&format!("- Node {:?}\n", node));
    }

    info
}
