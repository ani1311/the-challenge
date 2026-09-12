use crate::use_cases::ports::FriendshipRepository;


pub struct RejectFriendRequest<R> {
    friendship_repo: R,
}

pub struct RejectFriendRequestInput {

}

pub struct RejectFriendRequestOutput {

}

pub enum RejectFriendRequestError {

}

impl <R> RejectFriendRequest<R> where R: FriendshipRepository {
    pub async fn execute(&self, input: RejectFriendRequestInput) -> Result<RejectFriendRequestOutput, RejectFriendRequestError> {
        let op = RejectFriendRequestOutput{};
        Ok(op)
    }
}
