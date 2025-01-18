# tree-sitter-stack-graphs Learning Workspace

This workspace implements examples of generating and analyzing stack graphs from source code using the tree-sitter-stack-graphs library.

## Dependency Management

This project manages dependencies (tree-sitter, tree-sitter-stack-graphs, stack-graphs, tree-sitter-python) at the workspace level (workspace.dependencies). This approach provides the following benefits:

1. Version consistency: Ensures all crates use the same library versions
2. Maintainability: Centralizes dependency updates in one location
3. Build optimization: Prevents duplicate library resolution and reduces build time

Each crate references these dependencies using `.workspace = true`.

## Overview

tree-sitter-stack-graphs is a library that creates stack graphs using tree-sitter grammar. This library enables the following types of analysis:

- Name binding (variable resolution) in source code
- Module import relationship analysis
- Symbol reference tracking

## Setup

1. 依存関係（Dependencies）の追加
```toml
# 以下のバージョンは workspace.dependencies で管理されています
[dependencies]
tree-sitter-stack-graphs.workspace = true
tree-sitter.workspace = true
tree-sitter-python.workspace = true  # Select based on target language for analysis
stack-graphs.workspace = true
```

2. Build and Run
```bash
cargo build
cargo run
```

## 実装例

### Basic Graph Construction

tree-sitter-stack-graphs では、以下の基本的な API を使用して graph を構築します：

```rust
// Initialize graph
let mut graph = StackGraph::new();
let file_id = graph.get_or_create_file("example.py");

// Create symbols
let sys_symbol = graph.add_symbol("sys");
let path_symbol = graph.add_symbol("path");

// Create scope node
let scope_id = NodeID::new_in_file(file_id, 1);
let scope_node = graph.add_scope_node(scope_id, true);
```

### Stack Graph Node Types

tree-sitter-stack-graphs では、以下の node type を使用して graph を構築します：

- `scope` - Represents a scope node
- `push_symbol` - Node that pushes a symbol onto the stack
- `pop_symbol` - Node that pops a symbol from the stack
- `push_scoped_symbol` - Node that pushes a scoped symbol
- `pop_scoped_symbol` - Node that pops a scoped symbol

### Graph Operation Best Practices

1. Symbol Addition and Reference
   - `add_symbol` - Add a new symbol to the graph
   - Symbols are managed uniquely; identical symbol names share the same ID

2. Scope Node Creation
   - `NodeID::new_in_file` - Generate a node ID within a file
   - `add_scope_node` - Add a scope node (second argument boolean indicates exportable)

3. Graph Inspection and Debug
   - `iter_nodes()` - Traverse all nodes in the graph
   - Use `graph.iter_nodes().count()` to check node count
   - `node_symbol(node)` - Get symbol associated with node (different from symbols added via `add_symbol`)
   - `debug_info_for_node(node)` - Get debug info for node (useful for development troubleshooting)

```rust
// シンボルの取得と検査の例
for node in graph.iter_nodes() {
    // ノードのシンボルを取得
    if let Some(symbol) = graph.node_symbol(node) {
        println!("Node symbol: {:?}", symbol);
    }
    
    // デバッグ情報の取得
    if let Some(debug_info) = graph.debug_info_for_node(node) {
        println!("Debug info: {:?}", debug_info);
    }
}
```

注意点：
- `add_symbol`はグラフにシンボルを追加するのに対し、`node_symbol`は既存のノードからシンボルを取得します
- デバッグ情報は開発時のみ使用し、本番環境では使用を避けることを推奨します

### TSG (Tree-Sitter Graph) Rules

TSG rules define how to construct stack graphs from tree-sitter syntax trees.

Example: TSG rules for analyzing Python import statements and module references
```
(module) @prog {
    node module_scope
    attr (module_scope) type = "scope"

    ; Process module import statements
    (import_statement
        name: (dotted_name) @name) {
        node import_node
        attr (import_node) type = "pop_scoped_symbol"
        attr (import_node) symbol = (source-text @name)
        attr (import_node) is_definition
        
        edge (module_scope) -> (import_node)
    }

    ; Process module attribute access
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
```

### CLI Tools

tree-sitter-stack-graphs provides command-line tools:

```bash
# Install CLI
cargo install --features cli tree-sitter-stack-graphs

# Create index for source directory
tree-sitter-stack-graphs index SOURCE_DIR

# Search for definitions
tree-sitter-stack-graphs query definition SOURCE_PATH:LINE:COLUMN
```

## References

- [tree-sitter-stack-graphs API documentation](https://docs.rs/tree-sitter-stack-graphs/)
- [stack-graphs documentation](https://docs.rs/stack-graphs/)
- [tree-sitter documentation](https://tree-sitter.github.io/)
