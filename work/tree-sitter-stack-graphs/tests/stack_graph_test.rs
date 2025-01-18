use insta::assert_snapshot;
use tree_sitter_stack_graphs::{StackGraphLanguage, Variables, NoCancellation};
use stack_graphs::graph::StackGraph;
use tree_sitter_python;

#[test]
fn test_stack_graph_generation() {
    let python_source = r#"
import sys
print(sys.path)
    "#;

    let grammar = tree_sitter_python::language();
    let language = StackGraphLanguage::from_str(
        grammar,
        include_str!("../src/main.rs")
            .split("const STACK_GRAPH_RULES: &str = r#\"")
            .nth(1)
            .unwrap()
            .split("\"#;")
            .next()
            .unwrap(),
    ).unwrap();

    let mut stack_graph = StackGraph::new();
    let file_handle = stack_graph.get_or_create_file("test_sample.py");
    let globals = Variables::new();

    language.build_stack_graph_into(
        &mut stack_graph,
        file_handle,
        python_source,
        &globals,
        &NoCancellation,
    ).unwrap();

    assert_snapshot!(format!("Graph nodes: {}", stack_graph.iter_nodes().count()));
}
