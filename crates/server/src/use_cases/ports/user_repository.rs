use crate::domain::User;

pub enum UserRepositoryError {
    UserNotFound
}

pub trait UserRepository {
    async fn create_user(&self, username: String) -> Result<User, UserRepositoryError>;
    async fn lookup_user(&self, username: String) -> Result<User, UserRepositoryError>;
    async fn get_user_by_id(&self, user_id: String) -> Result<User, UserRepositoryError>;
}
