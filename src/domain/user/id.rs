use serde::Serialize;

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
