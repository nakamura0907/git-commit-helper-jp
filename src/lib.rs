mod git;
mod prompt;

use git::GitExecutor;
use prompt::Prompt;
use std::error::Error;

pub fn run() -> Result<(), Box<dyn Error>> {
    // 対話型プロンプトでコミットに関する情報の入力を促す
    let prompt = Prompt::new();
    let input = prompt.interact()?;

    // Gitコマンドを実行する
    let executor = GitExecutor::new(&input);
    executor.execute()?;

    Ok(())
}
