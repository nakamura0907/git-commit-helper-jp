use std::io;

use thiserror::Error;

/// Gitコマンド実行中のエラーを表す列挙型です。
#[derive(Error, Debug)]
pub enum GitError {
    /// コマンド実行時に起因するエラーです。
    #[error("コマンド実行時エラー: {0}")]
    ExecutionError(#[from] io::Error),
    /// コミット時に起因するエラーです。
    #[error("Gitコミットエラー: {0}")]
    CommitError(String),
}

/// `GitError`を`Result`型でラップした型です。
pub type GitResult<T> = Result<T, GitError>;
