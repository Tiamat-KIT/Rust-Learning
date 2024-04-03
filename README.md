# Rustをちゃんと勉強しよう 
### Rustは簡単か 

# そんなわけねえだろうが ェ～！？


### まず、Rustプロジェクト生成しようよ

```bash
cargo new [options] [ディレクトリ名]
```

#### オプション
##### 生成
- `--bin`
アプリケーションプロジェクトの作成
- `--lib`
ライブラリプロジェクトの作成
- `--name`
パッケージ名を指定する
- `--edition`
使用するRustのエディション指定
(2018か2021)
- `--vsc`
バージョン管理システムを指定
##### 表示
- `--verbose(-v)`
詳細なメッセージを表示する
(依存関係の警告やビルドスクリプトなど)
##### 共通
- `--help(-h)`
ヘルプ情報の表示

### `Cargo.toml`


```toml
[package]
name = "[ディレクトリ名]"
version = "0.1.0"
edition = "2021"

[dependencies]
```

### ビルドする
ビルドを行うときは`cargo build [options]`
デフォルトではデバッグ用ビルドになる
#### ビルドの違い
- デバッグ用
`cargo build` -> `target/debug`
- リリース用
`cargo build --release` -> `target/release`
    最適化されたバージョンがビルドされる
    - アプリケーションである場合
        実行ファイル(.exe)を生成する
    - ライブラリである場合
        ライブラリファイル(.rlib)が生成されます
    コンパイラには最適化レベルが指定でき0~3まで指定できる
    `Cargo.toml`の[profile.release]セクションに`opt-lebel`キーで指定する
#### オプション
