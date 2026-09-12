use crate::use_cases::ports::UserRepository;


pub struct RegisterUser<R> {
    user_repo: R,
}

pub struct RegisterUserInput {

}

pub struct RegisterUserOutput {

}

pub enum RegisterUserError {

}

impl <R> RegisterUser<R> where R: UserRepository{
    pub async fn execute(&self, input: RegisterUserInput) -> Result<RegisterUserOutput, RegisterUserError> {
        let op = RegisterUserOutput{};
        Ok(op)
    }
}
