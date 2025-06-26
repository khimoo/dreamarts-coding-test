# テストデータ

このディレクトリには、最長パス問題のアルゴリズムをテストするためのデータが含まれています。

## ファイル構成

- `longest_path_cases.yaml`: 正常なテストケース
- `error_cases.yaml`: エラーケースのテストデータ
- `README.md`: このファイル

## データ形式

### 正常なテストケース

各テストケースは以下の構造を持ちます：

```yaml
- name: "テストケース名"
  description: "テストケースの説明"
  input: |
    CSV形式のグラフデータ
  expected_path: [期待されるパスのノード列]
  expected_distance: 期待される距離
  expected_path_length: 期待されるパスの長さ
```

### エラーケース

各エラーケースは以下の構造を持ちます：

```yaml
- name: "エラーケース名"
  description: "エラーケースの説明"
  input: "不正な入力データ"
  expected_error: "期待されるエラーメッセージ"
```

## 入力形式

グラフデータは以下のCSV形式で記述します：

```
from,to,distance
```

例：
```
1,2,5.0
2,3,3.0
3,4,7.0
```

## 使用方法

Rustでの使用例：

```rust
use serde_yaml;
use std::fs;

#[derive(Debug, Deserialize)]
struct TestCase {
    name: String,
    description: String,
    input: String,
    expected_path: Vec<usize>,
    expected_distance: f64,
    expected_path_length: usize,
}

fn load_test_cases() -> Vec<TestCase> {
    let content = fs::read_to_string("tests/data/longest_path_cases.yaml").unwrap();
    let data: serde_yaml::Value = serde_yaml::from_str(&content).unwrap();
    // パース処理
}
```

## テストケースの種類

### 正常なテストケース

1. **基本的なケース**
   - 単純な直線パス
   - 分岐があるグラフ
   - サイクルを含むグラフ

2. **エッジケース**
   - 空の入力
   - 単一ノード
   - 孤立したノード

3. **特殊なケース**
   - 負の重み
   - ゼロの重み
   - 高精度の浮動小数点数
   - 大きな重み値

4. **パフォーマンステスト**
   - 大規模なグラフ

### エラーケース

1. **形式エラー**
   - フィールド数の不正
   - 空のフィールド

2. **数値エラー**
   - 無効な数値
   - 負のノードID
   - ゼロのノードID

## テストケースの追加

新しいテストケースを追加する場合は、以下の点に注意してください：

1. **一意な名前**: 既存のテストケースと重複しない名前を使用
2. **明確な説明**: テストケースの目的を明確に記述
3. **期待値の正確性**: 期待される結果が正しいことを確認
4. **エッジケースの考慮**: 境界値や特殊なケースをカバー

## 注意事項

- ノードIDは1から始まる正の整数を使用
- 距離は浮動小数点数で指定
- 同じノードを2回通らないパスを最長パスとする
- 複数の最長パスが存在する場合は、いずれか一つを期待値とする

## 補足
README.mdはclade codeで作成
