/// プロンプトを通じてユーザーから受け取った入力を表す構造体です。
pub struct UserResponse {
    commit_type: String,
    commit_message: String,
    commit_scope: Option<String>,
    commit_details: Option<String>,
    commit_reference: Option<String>,
}

impl UserResponse {
    pub fn new(
        commit_type: String,
        commit_message: String,
        commit_scope: Option<String>,
        commit_details: Option<String>,
        commit_reference: Option<String>,
    ) -> Self {
        UserResponse {
            commit_type,
            commit_message,
            commit_scope,
            commit_details,
            commit_reference,
        }
    }

    pub fn commit_type(&self) -> &str {
        &self.commit_type
    }

    pub fn commit_message(&self) -> &str {
        &self.commit_message
    }

    pub fn commit_scope(&self) -> Option<&str> {
        self.commit_scope.as_deref()
    }

    pub fn commit_details(&self) -> Option<&str> {
        self.commit_details.as_deref()
    }

    pub fn commit_reference(&self) -> Option<&str> {
        self.commit_reference.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::UserResponse;

    #[test]
    fn test_user_response() {
        let commit_type = "commit_type";
        let commit_message = "commit_message";
        let commit_scope = None;
        let commit_details = None;
        let commit_reference = None;

        let user_response = UserResponse::new(
            commit_type.to_string(),
            commit_message.to_string(),
            commit_scope,
            commit_details,
            commit_reference,
        );

        assert_eq!(user_response.commit_type(), commit_type);
        assert_eq!(user_response.commit_message(), commit_message);
        assert_eq!(user_response.commit_scope(), None);
        assert_eq!(user_response.commit_details(), None);
        assert_eq!(user_response.commit_reference(), None);
    }

    #[test]
    fn test_prompt_with_option() {
        let commit_type = "commit_type";
        let commit_message = "commit_message";
        let commit_scope = Some("commit_scope".to_string());
        let commit_details = Some("commit_details".to_string());
        let commit_reference = Some("commit_reference".to_string());

        let user_response = UserResponse::new(
            commit_type.to_string(),
            commit_message.to_string(),
            commit_scope,
            commit_details,
            commit_reference,
        );

        assert_eq!(user_response.commit_type(), commit_type);
        assert_eq!(user_response.commit_message(), commit_message);
        assert_eq!(user_response.commit_scope(), Some("commit_scope"));
        assert_eq!(user_response.commit_details(), Some("commit_details"));
        assert_eq!(user_response.commit_reference(), Some("commit_reference"));
    }
}
