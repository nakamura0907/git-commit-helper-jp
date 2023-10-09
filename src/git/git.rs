use std::process::Command;

use crate::prompt::UserResponse;

use super::git_error::{GitError, GitResult};

/// Gitコマンドを実行する構造体です。
///
/// `commit_message`をもとにコミットコマンドを実行します。
pub struct Git {
    commit_message: String,
}

impl Git {
    pub fn new(input: &UserResponse) -> Self {
        let mut parts = Vec::new();

        // コミットメッセージのヘッダーを作成
        let header = if let Some(scope) = input.commit_scope() {
            format!(
                "{}（{}）: {}",
                input.commit_type(),
                scope,
                input.commit_message()
            )
        } else {
            format!("{}: {}", input.commit_type(), input.commit_message())
        };
        parts.push(header);

        // コミットメッセージの詳細を作成
        if let Some(details) = input.commit_details() {
            parts.push(details.to_string());
        }

        // コミットメッセージの参照を作成
        if let Some(reference) = input.commit_reference() {
            parts.push(format!("参照: {}", reference));
        }

        let commit_message = parts.join("\n\n");

        Self { commit_message }
    }

    /// Gitコマンドを実行します。
    pub fn execute(&self) -> GitResult<()> {
        let output = self.run_git_command()?;
        self.handle_output(output)
    }

    /// 実際にGitコマンドを実行します。
    fn run_git_command(&self) -> Result<std::process::Output, GitError> {
        let result = Command::new("git")
            .arg("commit")
            .arg("-m")
            .arg(&self.commit_message)
            .output();

        result.map_err(GitError::from)
    }

    /// コマンドの出力を処理します。
    fn handle_output(&self, output: std::process::Output) -> GitResult<()> {
        let std_error = String::from_utf8_lossy(&output.stderr).replace("\n", "");
        if !output.status.success() {
            return Err(GitError::CommitError(std_error));
        }

        let std = String::from_utf8_lossy(&output.stdout);
        println!("{}", std);

        Ok(())
    }
}
