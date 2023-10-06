use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

#[derive(Debug)]
pub struct PromptError {
    message: String,
}

impl Display for PromptError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
impl Error for PromptError {}

impl PromptError {
    pub fn new(message: String) -> Self {
        PromptError { message }
    }
}

pub type PromptResult<T> = Result<T, PromptError>;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_prompt_error() {
        let prompt_error = PromptError::new("error".to_string());
        assert_eq!(prompt_error.to_string(), "error");
    }

    #[test]
    fn test_prompt_error_impl_error() {
        let prompt_error = PromptError::new("error".to_string());
        assert!(prompt_error.source().is_none());
    }
}
