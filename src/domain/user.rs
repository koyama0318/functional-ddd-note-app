#[derive(Debug)]
pub struct UserId {
    id: String,
}

impl UserId {
    pub fn id(&self) -> String {
        self.id.clone()
    }

    pub fn new(id: String) -> Self {
        UserId { id: id }
    }
}

#[derive(Debug)]
pub struct User {
    id: UserId,
    name: String,
}

impl User {
    pub fn new(id: UserId, name: String) -> Self {
        User { id: id, name: name }
    }
}

#[derive(Debug)]
pub enum UserError {
    NotFound,
}
