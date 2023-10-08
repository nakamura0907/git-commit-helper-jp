use inquire::{required, Select, Text};

use super::{
    prompt_error::{PromptError, PromptResult},
    UserResponse,
};

/// プロンプト構造体です。
///
/// プロンプトを実際に実行するメソッドを提供します。
pub struct Prompt {}

impl Prompt {
    pub fn new() -> Self {
        Prompt {}
    }

    /// プロンプトを実行してユーザーの入力を受け取ります。
    pub fn interact(&self) -> Result<UserResponse, PromptError> {
        let commit_type = Self::select_commit_type()?;
        let commit_message = Self::input_commit_message()?;

        let commit_scope = Self::input_commit_scope()?;
        let commit_details = Self::input_commit_details()?;
        let commit_reference = Self::input_commit_reference()?;

        Ok(UserResponse::new(
            commit_type,
            commit_message,
            commit_scope,
            commit_details,
            commit_reference,
        ))
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

    const COMMIT_SCOPE_PROMPT: &str = "コミットのスコープを入力してください（オプション）";
    const COMMIT_SCOPE_PLACEHOLDER: &str = "例: API";

    /// コミットのスコープを入力するプロンプトを実行します。
    fn input_commit_scope() -> PromptResult<Option<String>> {
        let ans: Result<String, inquire::InquireError> = Text::new(Self::COMMIT_SCOPE_PROMPT)
            .with_placeholder(Self::COMMIT_SCOPE_PLACEHOLDER)
            .prompt();

        ans.map(|scope| {
            if scope.trim().is_empty() {
                None
            } else {
                Some(scope)
            }
        })
        .map_err(PromptError::from)
    }

    const COMMIT_DETAILS_PROMPT: &str = "コミットの詳細を入力してください（オプション）";

    /// コミットの詳細を入力するプロンプトを実行します。
    fn input_commit_details() -> PromptResult<Option<String>> {
        let ans: Result<String, inquire::InquireError> =
            Text::new(Self::COMMIT_DETAILS_PROMPT).prompt();

        ans.map(|details| {
            if details.trim().is_empty() {
                None
            } else {
                Some(details)
            }
        })
        .map_err(PromptError::from)
    }

    const COMMIT_REFERENCE_PROMPT: &str = "コミットの参照を入力してください（オプション）";
    const COMMIT_REFERENCE_PLACEHOLDER: &str = "例: #123";

    /// コミットの参照を入力するプロンプトを実行します。
    fn input_commit_reference() -> PromptResult<Option<String>> {
        let ans: Result<String, inquire::InquireError> = Text::new(Self::COMMIT_REFERENCE_PROMPT)
            .with_placeholder(Self::COMMIT_REFERENCE_PLACEHOLDER)
            .prompt();

        ans.map(|reference| {
            if reference.trim().is_empty() {
                None
            } else {
                Some(reference)
            }
        })
        .map_err(PromptError::from)
    }
}
