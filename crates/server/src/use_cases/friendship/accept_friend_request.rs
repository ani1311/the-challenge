use crate::use_cases::ports::FriendshipRepository;


pub struct AcceptFriendRequest<R> {
    friendship_repo: R,
}

pub struct AcceptFriendRequestInput {

}

pub struct AcceptFriendRequestOutput {

}

pub enum AcceptFriendRequestError {

}

impl <R> AcceptFriendRequest<R> where R: FriendshipRepository {
    pub async fn execute(&self, input: AcceptFriendRequestInput) -> Result<AcceptFriendRequestOutput, AcceptFriendRequestError> {
        let op = AcceptFriendRequestOutput{};
        Ok(op)
    }
}
