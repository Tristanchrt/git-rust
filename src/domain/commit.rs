use chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(Debug)]
pub struct CommitToCreate {
    message: String,
}

impl CommitToCreate {
    pub fn new(message: String) -> CommitToCreate {
        CommitToCreate { message }
    }

    pub fn message(&self) -> &String {
        &self.message
    }

    pub fn default_parent_id() -> Uuid {
        Uuid::parse_str("00000000-0000-0000-0000-000000000000").unwrap()
    }
}

impl CommitToCreate {
    pub fn create(&self, parent_id: Uuid, branch_id: String, tree_hash: String) -> Commit {
        Commit {
            id: Uuid::new_v4(),
            parent_id,
            message: self.message.clone(),
            created_at: Self::now(),
            branch_id,
            tree_hash
        }
    }

    fn now() -> NaiveDateTime {
        NaiveDateTime::parse_from_str(
            &chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            "%Y-%m-%d %H:%M:%S",
        )
        .unwrap()
    }
}

#[derive(Debug, Clone)]
pub struct Commit {
    id: Uuid,
    parent_id: Uuid,
    message: String,
    created_at: NaiveDateTime,
    branch_id: String,
    tree_hash: String
}

impl PartialEq for Commit {
    fn eq(&self, other: &Self) -> bool {
        self.id.to_string() == other.id.to_string()
            && self.parent_id.to_string() == other.parent_id.to_string()
            && self.created_at == other.created_at
            && self.message == other.message
            && self.branch_id == other.branch_id
            && self.tree_hash == other.tree_hash
    }
}

impl Commit {
    pub fn new(
        id: Uuid,
        parent_id: Uuid,
        message: String,
        created_at: NaiveDateTime,
        branch_id: String,
        tree_hash: String
    ) -> Self {
        Self {
            id,
            parent_id,
            message,
            created_at,
            branch_id,
            tree_hash
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn parent_id(&self) -> Uuid {
        self.parent_id
    }

    pub fn message(&self) -> &String {
        &self.message
    }

    pub fn created_at(&self) -> NaiveDateTime {
        self.created_at
    }

    pub fn branch_id(&self) -> &String {
        &self.branch_id
    }

    pub  fn tree_hash(&self) -> &String {
        &self.tree_hash
    }
}
