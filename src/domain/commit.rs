use chrono::NaiveDateTime;


#[derive(Debug)]
pub struct Commit {
    id: String,
    parent_id: String,
    message: String,
    timestamp: NaiveDateTime
}

impl Commit {
    pub fn new(id: String, parent_id: String, message: String, timestamp: NaiveDateTime) -> Self {
        Self {
            id,
            parent_id,
            message,
            timestamp
        }
    }

    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn parent_id(&self) -> &String {
        &self.parent_id
    }

    pub fn message(&self) -> &String {
        &self.message
    }
}