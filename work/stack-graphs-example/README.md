# Stack Graphs サンプル実装

このワークスペースは、[stack-graphs](https://github.com/github/stack-graphs)ライブラリの学習と実装例を提供することを目的としています。

## 概要

Stack Graphsは、プログラミング言語における名前解決を効率的に行うためのライブラリです。以下の特徴があります：

- インクリメンタルな名前解決の実現
- Graph-basedアプローチによる効率的な解析
- 複数言語対応の名前解決framework

## 基本概念

### Stack Graph

Stack graphは以下の要素で構成されています：

1. **File handle**: ソースコードファイルを表現
2. **Symbol**: 変数や関数などの名前を表現
3. **Scope node**: 名前の有効範囲を表現

### 名前解決の仕組み

1. ソースコードからgraphを構築
2. Symbolとscopeの関係を定義
3. Pathを探索して名前解決を実行

## 実装例

このプロジェクトでは、基本的なstack graphの構築方法を示しています：

```rust
use stack_graphs::graph::{StackGraph, NodeID};

fn main() {
    // スタックグラフの作成
    let mut graph = StackGraph::new();
    
    // ファイルハンドルの作成
    let file_id = graph.get_or_create_file("example.rs");
    
    // シンボルの作成
    let _a_symbol = graph.add_symbol("A");
    let _foo_symbol = graph.add_symbol("foo");
    
    // スコープノードの作成
    let scope_id = NodeID::new_in_file(file_id, 1);
    let scope_node = graph.add_scope_node(scope_id, true);
}
```

## セットアップ

1. Rustツールチェーンのインストール
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. プロジェクトのクローン
```bash
git clone https://github.com/HikaruEgashira/rust-workspaces.git
cd rust-workspaces
```

## 実行方法

```bash
# ビルド
cargo build -p stack-graphs-example

# 実行
cargo run -p stack-graphs-example

# リント
cargo clippy -p stack-graphs-example
```

## 出力例

```
Created stack graph with:
  - File: example.rs
  - Symbols: A, foo
  - Scope node: Some(Handle { index: 3 })

Stack-Graphs Example Completed
```

## 参考リンク

- [Stack Graphs GitHub](https://github.com/github/stack-graphs)
- [Stack Graphs Documentation](https://docs.rs/stack-graphs/)

## ライセンス

このプロジェクトはMITライセンスの下で公開されています。
