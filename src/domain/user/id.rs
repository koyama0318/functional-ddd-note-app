use super::error::UserError;
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize, Debug, Clone)]
pub struct UserId {
    id: String,
}

impl UserId {
    pub fn default() -> Self {
        UserId {
            id: Uuid::new_v4().to_string(),
        }
    }

    pub fn new(id: String) -> Result<Self, UserError> {
        match uuid::Uuid::parse_str(&id) {
            Ok(_) => Ok(UserId { id }),
            Err(_) => Err(UserError::ValidationError),
        }
    }

    pub fn id(&self) -> String {
        self.id.clone()
    }
}
