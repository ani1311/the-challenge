use crate::use_cases::ports::FriendshipRepository;


pub struct SendFriendRequest<R> {
    friendship_repo: R,
}

pub struct SendFriendRequestInput {

}

pub struct SendFriendRequestOutput {

}

pub enum SendFriendRequestError {

}

impl <R> SendFriendRequest<R> where R: FriendshipRepository {
    pub async fn execute(&self, input: SendFriendRequestInput) -> Result<SendFriendRequestOutput, SendFriendRequestError> {
        let op = SendFriendRequestOutput{};
        Ok(op)
    }
}
