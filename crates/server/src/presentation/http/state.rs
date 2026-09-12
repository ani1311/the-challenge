use crate::{infrastructure::jwt_service::JwtService, persistence::sqlx_user_repository::SqlxUserRepository};

#[derive(Clone)]
pub struct AppState {
    user_repo: SqlxUserRepository,
    auth: JwtService,
}

impl AppState {
    pub fn new(user_repo: SqlxUserRepository, auth: JwtService) -> Self {
        Self { user_repo, auth }
    }

    pub fn user_repo(&self) -> SqlxUserRepository {
        self.user_repo.clone()
    }

    pub fn auth(&self) -> JwtService {
        self.auth.clone()
    }
}
