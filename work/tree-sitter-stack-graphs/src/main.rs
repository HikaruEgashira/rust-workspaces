use tree_sitter_stack_graphs::{StackGraphLanguage, Variables, NoCancellation};
use stack_graphs::graph::StackGraph;
use tree_sitter_python;

// TSGルールの定義
// 基本的なPythonのimport文とモジュール参照を解析するルール
const STACK_GRAPH_RULES: &str = r#"
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 解析対象のPythonソースコード
    let python_source = r#"
import sys
print(sys.path)
    "#;

    // tree-sitter-pythonのLanguageを取得
    let grammar = tree_sitter_python::language();

    // tree-sitter-stack-graphsのStackGraphLanguageを作成
    let mut language = StackGraphLanguage::from_str(grammar, STACK_GRAPH_RULES)?;

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
