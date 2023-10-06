use std::{error::Error, process::Command};

use crate::prompt::PromptInput;

pub fn execute(input: PromptInput) -> Result<(), Box<dyn Error>> {
    let text = format!("{}: {}", input.commit_type(), input.commit_message());
    let commit_command = format!("git commit -m '{}'", text);

    let output = if cfg!(target_os = "windows") {
        Command::new("cmd").args(["/C", &commit_command]).output()
    } else {
        Command::new("sh").arg("-c").arg(commit_command).output()
    };

    match output {
        Ok(output) => {
            let std = String::from_utf8_lossy(&output.stderr).replace("\n", "");
            if !output.status.success() {
                return Err(From::from(format!("{}", std)));
            }

            println!("{}", std);

            Ok(())
        }
        Err(e) => Err(From::from(format!("{}", e))),
    }
}
