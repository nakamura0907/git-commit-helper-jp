mod prompt;

use prompt::Prompt;
use std::error::Error;

pub fn run() -> Result<(), Box<dyn Error>> {
    let prompt = Prompt::new();

    let _input = prompt.execute();
    if let Err(e) = _input {
        return Err(Box::new(e));
    }

    Ok(())
}
