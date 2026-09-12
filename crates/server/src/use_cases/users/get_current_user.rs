use crate::{domain::User, use_cases::ports::{UserRepository, UserRepositoryError}};

pub struct GetMe<R> {
    user_repo: R
}

pub struct GetCurrentUserInput {
    pub user_id: String
}

pub struct GetCurrentUserOutput {
    pub user: User,
}

pub enum GetCurrentUserError {
    UserNotFound,
}

impl From<UserRepositoryError> for GetCurrentUserError {
    fn from(error: UserRepositoryError) -> Self {
        match error {
            UserRepositoryError::UserNotFound => Self::UserNotFound,
        }
    }
}

impl <R> GetMe<R> where R: UserRepository {

    pub fn new(user_repo: R) -> Self {
        Self { user_repo }
    }

    pub async fn execute(&self, input: GetCurrentUserInput) -> Result<GetCurrentUserOutput, GetCurrentUserError> {
        let user = self.user_repo.get_user_by_id(input.user_id).await?;
        Ok(GetCurrentUserOutput { user })
    }
}
