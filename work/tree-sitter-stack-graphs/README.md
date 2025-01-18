# tree-sitter-stack-graphs学習用ワークスペース

このワークスペースでは、tree-sitter-stack-graphsライブラリを使用し、ソースコードのスタックグラフを生成・解析するサンプルを実装しています。

## 概要

tree-sitter-stack-graphsは、tree-sitterの文法を使用してstack graphsを作成するためのライブラリです。このライブラリを使用することで、以下のような解析が可能になります：

- ソースコード内の名前解決（name binding）
- モジュールのimport関係の解析
- シンボル参照の追跡

## セットアップ

1. 依存関係の追加
```toml
[dependencies]
tree-sitter-stack-graphs = "0.10"
tree-sitter = "0.20"
tree-sitter-python = "0.20"  # 解析対象の言語に応じて選択
stack-graphs = "0.10"
```

2. ビルドと実行
```bash
cargo build
cargo run
```

## 実装例

### Stack Graph Node Types

tree-sitter-stack-graphsでは、以下のnode typeを使用してグラフを構築します：

- `scope` - スコープを表すnode
- `push_symbol` - シンボルをスタックにプッシュするnode
- `pop_symbol` - シンボルをスタックからポップするnode
- `push_scoped_symbol` - スコープ付きシンボルをプッシュするnode
- `pop_scoped_symbol` - スコープ付きシンボルをポップするnode

### TSG (Tree-Sitter Graph) Rules

TSGルールは、tree-sitterの構文木からstack graphを構築するための規則を定義します。

例：Pythonのimport文とモジュール参照を解析するTSGルール
```
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
```

### CLIツール

tree-sitter-stack-graphsには、コマンドラインツールも用意されています：

```bash
# CLIのインストール
cargo install --features cli tree-sitter-stack-graphs

# ソースディレクトリのインデックス作成
tree-sitter-stack-graphs index SOURCE_DIR

# 定義の検索
tree-sitter-stack-graphs query definition SOURCE_PATH:LINE:COLUMN
```

## 参考資料

- [tree-sitter-stack-graphs API documentation](https://docs.rs/tree-sitter-stack-graphs/)
- [stack-graphs documentation](https://docs.rs/stack-graphs/)
- [tree-sitter documentation](https://tree-sitter.github.io/)
