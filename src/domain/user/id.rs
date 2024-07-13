use serde::Serialize;

use super::error::UserError;

#[derive(Serialize, Debug)]
pub struct UserId {
    id: u64,
}

impl UserId {
    pub fn new(id: u64) -> Self {
        UserId { id: id }
    }

    pub fn id(&self) -> u64 {
        self.id.clone()
    }
}

pub fn user_id(id: u64) -> Result<UserId, UserError> {
    Ok(UserId::new(id))
}
