use stack_graphs::graph::StackGraph;
use tree_sitter_python::LANGUAGE;
use tree_sitter_stack_graphs::{NoCancellation, StackGraphLanguage, Variables};

mod dot_export;

// TSGルールの定義
// 基本的なPythonのimport文とモジュール参照を解析するルール
const STACK_GRAPH_RULES: &str = r#"
(module) @__tsg__full_match {
    node scope_node
    attr (scope_node) type = "scope"
}

(import_statement name: (dotted_name) @name) {
    node scope_node
    attr (scope_node) type = "scope"

    node import_ref
    attr (import_ref) type = "pop_symbol"
    attr (import_ref) symbol = (source-text @name)
    attr (import_ref) is_definition
    edge scope_node -> import_ref
}

(attribute object: (identifier) @obj attribute: (identifier) @attr) {
    node scope_node
    attr (scope_node) type = "scope"

    node ref_node
    attr (ref_node) type = "push_symbol"
    attr (ref_node) symbol = (source-text @obj)
    
    node attr_node
    attr (attr_node) type = "pop_symbol"
    attr (attr_node) symbol = (source-text @attr)
    edge ref_node -> attr_node
}
"#;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 解析対象のPythonソースコード
    let python_source = r#"
import sys
print(sys.path)
    "#;

    // tree-sitter-pythonのLanguageを取得
    let grammar = LANGUAGE.into();

    // tree-sitter-stack-graphsのStackGraphLanguageを作成
    let language = StackGraphLanguage::from_str(grammar, STACK_GRAPH_RULES)?;

    // StackGraphインスタンス生成
    let mut stack_graph = StackGraph::new();
    let file_handle = stack_graph.get_or_create_file("sample.py");

    // グローバル変数（ファイル名など）を設定
    let globals = Variables::new();

    // スタックグラフの構築
    language.build_stack_graph_into(
        &mut stack_graph,
        file_handle,
        python_source,
        &globals,
        &NoCancellation,
    )?;

    // 生成されたグラフの情報を出力
    println!("Graph has {} nodes", stack_graph.iter_nodes().count());
    println!("Generated stack graph for Python code analysis");

    // DOT形式でグラフを出力
    let dot_contents = dot_export::to_dot(&stack_graph);
    println!("\nDOT representation:\n{}", dot_contents);

    Ok(())
}
