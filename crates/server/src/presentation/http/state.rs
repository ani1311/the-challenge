use crate::{persistence::sqlx_user_repository::SqlxUserRepository, use_cases::ports::UserRepository};


#[derive(Clone)]
pub struct AppState {
    user_repo: SqlxUserRepository
}

impl AppState {
    pub fn new(user_repo: SqlxUserRepository) -> Self {
        Self { user_repo }
    }

    pub fn user_repo(&self) -> SqlxUserRepository{
        self.user_repo.clone()
    }
}
