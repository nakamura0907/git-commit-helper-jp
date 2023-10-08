use inquire::{required, Select, Text};

use super::prompt_error::{PromptError, PromptResult};

/// プロンプト構造体です。
///
/// プロンプトを実際に実行するメソッドを提供します。
pub struct Prompt {}

impl Prompt {
    pub fn new() -> Self {
        Prompt {}
    }

    /// プロンプトを実行してユーザーの入力を受け取ります。
    pub fn execute(&self) -> Result<PromptInput, PromptError> {
        let commit_type = Self::select_commit_type()?;
        let commit_message = Self::input_commit_message()?;

        Ok(PromptInput {
            commit_type,
            commit_message,
        })
    }

    const COMMIT_TYPE_OPTIONS: [&str; 6] = [
        "機能追加",
        "バグ修正",
        "テスト",
        "リファクタリング",
        "ドキュメンテーション",
        "その他",
    ];
    const COMMIT_TYPE_PROMPT: &str = "コミットの種類を選択してください";

    /// コミットの種類を選択するプロンプトを実行します。
    fn select_commit_type() -> PromptResult<String> {
        let ans: Result<&str, inquire::InquireError> =
            Select::new(Self::COMMIT_TYPE_PROMPT, Self::COMMIT_TYPE_OPTIONS.to_vec()).prompt();

        ans.map(|choice| choice.to_string())
            .map_err(PromptError::from)
    }

    const COMMIT_MESSAGE_PROMPT: &str = "コミットメッセージを入力してください";

    /// コミットメッセージを入力するプロンプトを実行します。
    fn input_commit_message() -> PromptResult<String> {
        let ans: Result<String, inquire::InquireError> = Text::new(Self::COMMIT_MESSAGE_PROMPT)
            .with_validator(required!())
            .prompt();

        ans.map_err(PromptError::from)
    }
}

/// プロンプトの入力値を保持する構造体です。
pub struct PromptInput {
    commit_type: String,
    commit_message: String,
}

impl PromptInput {
    pub fn commit_type(&self) -> &str {
        &self.commit_type
    }

    pub fn commit_message(&self) -> &str {
        &self.commit_message
    }
}
