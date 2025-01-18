use tree_sitter_stack_graphs::{StackGraphLanguage, Variables, NoCancellation};
use stack_graphs::graph::StackGraph;
use tree_sitter_python::LANGUAGE;

// TSGルールの定義
// 基本的なPythonのimport文とモジュール参照を解析するルール
const STACK_GRAPH_RULES: &str = r#"
(module) @__tsg__full_match {
    node root
    attr (root) type = "scope"
    attr (root) is_definition

    (import_statement
        name: (dotted_name) @name) {
        node import_ref
        attr (import_ref) type = "reference"
        attr (import_ref) symbol = (source-text @name)
        edge (root) -> (import_ref)
    }

    (attribute
        object: (identifier) @obj
        attribute: (identifier) @attr) {
        node attr_ref
        attr (attr_ref) type = "reference"
        attr (attr_ref) symbol = (source-text @attr)
        edge (root) -> (attr_ref)
    }
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
    
    Ok(())
}
