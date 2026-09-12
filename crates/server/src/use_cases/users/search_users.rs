use crate::use_cases::ports::UserRepository;



pub struct SearchUser<R> {
    user_repo: R,
}

pub struct SearchUserInput {

}

pub struct SearchUserOutput {

}

pub enum SearchUserError {

}

impl <R> SearchUser<R> where R: UserRepository{
    pub async fn execute(&self, input: SearchUserInput) -> Result<SearchUserOutput, SearchUserError> {
        let op = SearchUserOutput{};
        Ok(op)
    }
}
