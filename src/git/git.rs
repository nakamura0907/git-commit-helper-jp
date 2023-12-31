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
        let header = match input.commit_scope() {
            Some(scope) => format!(
                "{}（{}）: {}",
                input.commit_type(),
                scope,
                input.commit_message()
            ),
            None => format!("{}: {}", input.commit_type(), input.commit_message()),
        };

        let parts: Vec<_> = vec![
            header,
            input.commit_details().unwrap_or_default().to_string(),
            input
                .commit_reference()
                .map(|r| format!("参照: {}", r))
                .unwrap_or_default(),
        ]
        .into_iter()
        .filter(|s| !s.is_empty())
        .collect();

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

        // Gitコマンドの出力を表示する
        let std = String::from_utf8_lossy(&output.stdout);
        println!("{}", std);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::prompt::UserResponse;

    use super::Git;

    #[test]
    fn test_git_new() {
        let respones = UserResponse::new(
            "commit_type".to_string(),
            "commit_message".to_string(),
            None,
            None,
            None,
        );
        let git = Git::new(&respones);

        assert_eq!(
            git.commit_message,
            "commit_type: commit_message".to_string()
        );
    }

    #[test]
    fn test_git_new_with_option() {
        let respones = UserResponse::new(
            "commit_type".to_string(),
            "commit_message".to_string(),
            Some("commit_scope".to_string()),
            Some("commit_details".to_string()),
            Some("commit_reference".to_string()),
        );
        let git = Git::new(&respones);

        assert_eq!(
            git.commit_message,
            "commit_type（commit_scope）: commit_message\n\ncommit_details\n\n参照: commit_reference".to_string()
        );
    }
}
