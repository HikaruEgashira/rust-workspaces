use stack_graphs::arena::Handle;
use stack_graphs::graph::{Node, StackGraph};

/// Converts a StackGraph to DOT format for visualization
///
/// # Arguments
/// * `graph` - Reference to the StackGraph to visualize
///
/// # Returns
/// A String containing the DOT format representation of the graph
pub fn to_dot(graph: &StackGraph) -> String {
    let mut dot = String::from("digraph StackGraph {\n");

    // グラフの属性を設定
    dot.push_str("    // Graph attributes\n");
    dot.push_str("    graph [rankdir=LR];\n");
    dot.push_str("    node [shape=box, style=rounded];\n\n");

    // ノードの出力
    dot.push_str("    // Nodes\n");
    for node_handle in graph.iter_nodes() {
        let node_attrs = get_node_attributes(graph, node_handle);
        dot.push_str(&format!(
            "    \"{}\" [{}];\n",
            node_to_string(node_handle),
            node_attrs
        ));
    }

    // エッジの出力
    dot.push_str("\n    // Edges\n");
    for node_handle in graph.iter_nodes() {
        for edge in graph.outgoing_edges(node_handle) {
            dot.push_str(&format!(
                "    \"{}\" -> \"{}\";\n",
                node_to_string(node_handle),
                node_to_string(edge.sink)
            ));
        }
    }

    dot.push_str("}\n");
    dot
}

/// ノードの文字列表現を生成
fn node_to_string(node: Handle<Node>) -> String {
    format!("{:?}", node)
}

/// ノードの属性を取得
fn get_node_attributes(graph: &StackGraph, node: Handle<Node>) -> String {
    let mut attrs = Vec::new();

    // ノードの種類に応じて色を設定
    // Get node and determine color based on its type
    let color = match &graph[node] {
        Node::Scope(_) => "lightblue",
        Node::PushSymbol(_) => "lightgreen",
        Node::PopSymbol(_) => "lightpink",
        Node::PushScopedSymbol(_) => "lightgreen",
        Node::PopScopedSymbol(_) => "lightpink",
        Node::DropScopes(_) => "yellow",
        Node::JumpTo(_) => "orange",
        Node::Root(_) => "purple",
        _ => "white",
    };
    attrs.push(format!("fillcolor=\"{}\"", color));
    attrs.push("style=\"filled\"".to_string());

    // ノード情報を追加
    attrs.push(format!("label=\"{:?}\"", node));

    attrs.join(", ")
}
