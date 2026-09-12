use crate::use_cases::ports::UserRepository;


pub struct GetCurrentUser<R> {
    user_repo: R
}

pub struct GetCurrentUserInput {

}

pub struct GetCurrentUserOutput {

}

pub enum GetCurrentUserError {

}

impl <R> GetCurrentUser<R> where R: UserRepository {
    pub async fn execute(&self, input: GetCurrentUserInput) -> Result<GetCurrentUserOutput, GetCurrentUserError> {
        let op = GetCurrentUserOutput{};
        Ok(op)
    }
}
