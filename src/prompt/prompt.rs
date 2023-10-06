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

    fn select_commit_type() -> PromptResult<String> {
        let options = vec!["機能追加", "バグ修正", "その他"];
        let ans: Result<&str, inquire::InquireError> =
            Select::new("コミット種別を選択してください", options).prompt();

        match ans {
            Ok(choice) => Ok(choice.to_string()),
            Err(e) => Err(PromptError::new(e.to_string())),
        }
    }

    fn input_commit_message() -> PromptResult<String> {
        let ans: Result<String, inquire::InquireError> = Text::new("コミットメッセージ")
            .with_validator(required!())
            .prompt();

        match ans {
            Ok(ans) => Ok(ans.to_string()),
            Err(e) => Err(PromptError::new(e.to_string())),
        }
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
