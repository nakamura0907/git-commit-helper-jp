use std::{error::Error, process::Command};

use crate::prompt::PromptInput;

/// Gitコマンドを実行する
pub fn execute(input: PromptInput) -> Result<(), Box<dyn Error>> {
    // コミットメッセージを作成する
    let text = format!("{}: {}", input.commit_type(), input.commit_message());
    let commit_command = format!("git commit -m '{}'", text);

    // コミットを実行する
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd").args(["/C", &commit_command]).output()
    } else {
        Command::new("sh").arg("-c").arg(&commit_command).output()
    };

    match output {
        Ok(output) => {
            // 標準エラー出力を確認する
            let std_error = String::from_utf8_lossy(&output.stderr).replace("\n", "");
            if !output.status.success() {
                return Err(From::from(format!("{:#?}", std_error)));
            }

            // 標準出力を表示する
            let std = String::from_utf8_lossy(&output.stdout);
            println!("{}", std);

            Ok(())
        }
        Err(e) => Err(From::from(format!("{}", e))),
    }
}
