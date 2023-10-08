mod git;
mod prompt;

use git::GitExecutor;
use prompt::Prompt;
use std::error::Error;

pub fn run() -> Result<(), Box<dyn Error>> {
    // 対話型プロンプトを実行する
    let prompt = Prompt::new();
    let input = prompt.execute()?;

    // Gitコマンドを実行する
    let executor = GitExecutor::new(&input);
    executor.execute()?;

    Ok(())
}
