
pub struct User {
    id: String,
    username: String,
}

impl User {
    pub fn new(username: String) -> Self {
        Self { id: "test".to_string(), username }
    }

    pub fn id(&self) -> String {
        self.id.clone()
    }

    pub fn username(&self) -> String {
        self.username.clone()
    }
}
