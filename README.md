# Git Commit Hepler JP

Git Commit Helper JPはGitのコミットメッセージ作成を助けてくれるCLIツールです。

対話型プロンプトを通じてコミット種別の選択やコミットメッセージの入力を行い、最終的にはコミットを実行します。

## Install

現バージョンでは、以下の手順でインストールできます。

（将来的にはインストールスクリプトを用意する予定です）

1. このプロジェクトをクローンする
2. プロジェクトのディレクトリに移動する
3. プロジェクトをビルドする

    ```bash
    $ cargo build --release
    ```

`target/release/`配下に実行可能ファイルが生成されます。

## Usage

`target/release/`配下の実行可能ファイルを実行することでCLIツールを使用できます。

（将来的には`git chjp`コマンドで利用可能になる予定です）

現バージョンでも`git --exec-path`配下に配置することで上記コマンドで実行できるようになります。

### Command line options

| パラメータ     | 型    | 概要         |
|----------------|-------|--------------|
| `-h`, `--help`     | null  | Print help  |
| `-V`, `--version`  | null  | Print version |
