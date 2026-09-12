use crate::domain::User;

pub enum UserRepositoryError {
    UserNotFound
}

pub trait UserRepository {
    async fn create_user(&self, username: String) -> Result<User, UserRepositoryError>;
    async fn lookup_user(&self, username: String) -> Result<User, UserRepositoryError>;
}
