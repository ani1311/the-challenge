use crate::use_cases::ports::{UserRepository, UserRepositoryError};


pub struct RegisterUser<R> {
    user_repo: R,
}

pub struct RegisterUserInput {
    pub name: String
}

pub struct RegisterUserOutput {
    pub user_id: String
}

pub enum RegisterUserError {
    Failed
}

impl From<UserRepositoryError> for RegisterUserError {
    fn from(error: UserRepositoryError) -> Self {
        Self::Failed
    }
}

impl <R> RegisterUser<R> where R: UserRepository{

    pub fn new(user_repo: R) -> Self {
        Self { user_repo }
    }

    pub async fn execute(&self, input: RegisterUserInput) -> Result<RegisterUserOutput, RegisterUserError> {
        let user = self.user_repo.create_user(input.name).await?;
        Ok(RegisterUserOutput { user_id: user.id() })
    }
}
