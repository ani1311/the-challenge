use crate::domain::User;

pub enum UserRepositoryError {

}

pub trait UserRepository {
    async fn create_user(&self, username: String) -> Result<User, UserRepositoryError>;
}
