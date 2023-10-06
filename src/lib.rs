mod git;
mod prompt;

use prompt::Prompt;
use std::error::Error;

pub fn run() -> Result<(), Box<dyn Error>> {
    let prompt = Prompt::new();

    let input = prompt.execute();
    if let Err(e) = input {
        return Err(Box::new(e));
    }

    if let Err(e) = git::execute(input?) {
        return Err(e);
    }

    Ok(())
}
