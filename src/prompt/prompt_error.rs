use inquire::InquireError;
use thiserror::Error;

/// プロンプト処理中のエラーを表す列挙型です。
#[derive(Error, Debug)]
pub enum PromptError {
    /// inquireパッケージに起因するエラーです。
    #[error("プロンプトエラー: {0}")]
    InquireError(#[from] InquireError),
}

/// `PromptError`を`Result`型でラップした型です。
pub type PromptResult<T> = Result<T, PromptError>;
