# Rust Stdlib

Rustで構築する汎用標準ライブラリ群。

本プロジェクトは、数学・数量・時間・データ・実行基盤など、現実世界を型安全に表現するための基礎ライブラリを構築することを目的とする。

## Design Philosophy

単なる計算ライブラリではなく、

```
現実世界
   ↓
モデル化
   ↓
数学的表現
   ↓
型安全な実装
   ↓
制御・応用システム
```

という流れを支える基盤を目指す。

## Workspace Structure

```
stdlib/
├ core/       # 共通基盤
├ math/       # 数学基盤
├ quantity/   # 単位付き量
├ time/       # 時間表現
├ data/       # データ処理
├ runtime/    # 実行基盤
└ finance/    # 金融モデル
```

## math

現在の主要実装。

### Vector

固定長ジェネリックベクトル。

対応：

* 加算
* 減算
* スカラー倍
* スカラー除算
* 内積 (dot product)
* 大きさ (magnitude)

例：

```rust
let a = Vector::new([1, 2, 3]);
let b = Vector::new([4, 5, 6]);

let c = a + b;
let dot = a.dot(&b);
```

### Algebra Traits

基本的な数学能力をtraitとして分離。

対応：

* Zero
* One
* Sqrt
* Identity
* Inverse

ジェネリクスとtrait境界により、型が持つ能力に応じて演算を提供する。

### Boundary

値の境界条件を表現。

現在：

* Bound
* Threshold
* Interval

対応：

例：

```
x < 30

40 <= x <= 60
```

のような数学的条件を型として表現する。

## Current Milestone

`math-foundation-v0.1.0`

達成：

* 数値型基盤
* Vector基盤
* 基本演算
* 内積
* Magnitude
* Boundary基盤
* Threshold
* Interval

## Future Direction

今後は以下を拡張予定。

### Quantity

値に単位を持たせる。

例：

```
20
↓
20 %
20 ℃
20 L/min
```

### Engineering Layer

数学モデルを現実世界の意味へ変換する。

例：

```
40 <= moisture <= 60
        ↓
Optimal moisture
        ↓
Irrigation control
```

### Domain Layer

農業・制御など、実際の用途へ展開する。

## Development

Build:

```bash
cargo build
```

Test:

```bash
cargo test
```

Workspace test:

```bash
cargo test --workspace
```

## License

TBD
