use crate::{domain::User, use_cases::ports::{self, UserRepositoryError}};


#[derive(Clone)]
pub struct SqlxUserRepository {
    pool: String
}

impl SqlxUserRepository {
    pub fn new() -> SqlxUserRepository {
        SqlxUserRepository { pool: "".to_string() }
    }
}

impl ports::UserRepository for SqlxUserRepository {
    async fn create_user(&self, username: String) -> Result<User, UserRepositoryError> {
        Ok(User::new(username))
    }
}
