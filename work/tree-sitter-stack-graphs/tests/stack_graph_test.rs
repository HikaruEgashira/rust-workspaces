use insta::assert_snapshot;
use tree_sitter_stack_graphs::{StackGraphLanguage, Variables, NoCancellation};
use stack_graphs::graph::StackGraph;
use tree_sitter_python;

const TEST_RULES: &str = r#"
(module) @prog {
    node module_scope
    attr (module_scope) type = "scope"

    ; モジュールのインポート文を処理
    (import_statement
        name: (dotted_name) @name) {
        node import_node
        attr (import_node) type = "pop_scoped_symbol"
        attr (import_node) symbol = (source-text @name)
        attr (import_node) is_definition
        
        edge (module_scope) -> (import_node)
    }

    ; モジュール属性アクセスを処理
    (attribute
        object: (identifier) @obj
        attribute: (identifier) @attr) {
        node ref_node
        attr (ref_node) type = "push_scoped_symbol"
        attr (ref_node) symbol = (source-text @obj)
        
        node attr_node
        attr (attr_node) type = "pop_scoped_symbol"
        attr (attr_node) symbol = (source-text @attr)
        
        edge (ref_node) -> (attr_node)
    }
}
"#;

#[test]
fn test_basic_import() {
    let python_source = r#"import sys"#;
    let stack_graph = create_stack_graph(python_source);
    assert_snapshot!(format_graph_info(&stack_graph));
}

#[test]
fn test_module_attribute_access() {
    let python_source = r#"
import sys
print(sys.path)
    "#;
    let stack_graph = create_stack_graph(python_source);
    assert_snapshot!(format_graph_info(&stack_graph));
}

#[test]
fn test_multiple_imports() {
    let python_source = r#"
import os
import sys
import json
    "#;
    let stack_graph = create_stack_graph(python_source);
    assert_snapshot!(format_graph_info(&stack_graph));
}

fn create_stack_graph(source: &str) -> StackGraph {
    let grammar = tree_sitter_python::LANGUAGE.into();
    let language = StackGraphLanguage::from_str(grammar, TEST_RULES).unwrap();
    let mut stack_graph = StackGraph::new();
    let file_handle = stack_graph.get_or_create_file("test_sample.py");
    let globals = Variables::new();

    language.build_stack_graph_into(
        &mut stack_graph,
        file_handle,
        source,
        &globals,
        &NoCancellation,
    ).unwrap();

    stack_graph
}

fn format_graph_info(graph: &StackGraph) -> String {
    let mut info = String::new();
    info.push_str(&format!("Total nodes: {}\n", graph.iter_nodes().count()));
    
    // ノードタイプごとの数を集計
    let mut scope_nodes = 0;
    let mut push_nodes = 0;
    let mut pop_nodes = 0;

    for node in graph.iter_nodes() {
        if let Some(debug_info) = graph.node_debug_info(node) {
            match debug_info.as_str() {
                s if s.contains("scope") => scope_nodes += 1,
                s if s.contains("push_scoped_symbol") => push_nodes += 1,
                s if s.contains("pop_scoped_symbol") => pop_nodes += 1,
                _ => {}
            }
        }
    }

    info.push_str(&format!("Scope nodes: {}\n", scope_nodes));
    info.push_str(&format!("Push symbol nodes: {}\n", push_nodes));
    info.push_str(&format!("Pop symbol nodes: {}\n", pop_nodes));

    // シンボル情報を追加
    info.push_str("\nNode symbols:\n");
    for node in graph.iter_nodes() {
        if let Some(symbol) = graph.get_node_symbol(node) {
            info.push_str(&format!("- {}\n", symbol));
        }
    }

    info
}
