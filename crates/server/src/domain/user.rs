
pub struct User {
    id: String,
    name: String
}

impl User {
    pub fn new(name: String) ->Self {
        Self { id: "test".to_string(), name }
    }

    pub fn id(&self)  -> String {
        self.id.clone()
    }
}
