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
    pub fn create(&self, parent_id: Uuid, branch_id: String) -> Commit {
        Commit {
            id: Uuid::new_v4(),
            parent_id,
            message: self.message.clone(),
            created_at: Self::now(),
            branch_id,
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

// TODO add hash Tree : https://chatgpt.com/c/67e24596-7ef0-8012-80b9-745c677b5838
// TODO 1 commit = 1 Tree and use SHA1 for Hash
// TODO blob are just blob and tree are a metadata structure that's look like this
// 100644 blob def4567890abcdef1234567890abcdef1234 file2.txt
// 100644 blob ghi7894567890abcdef1234567890abcdef56 file3.txt
// objects/
    // ab/
        // cdef1234567890abcdef1234567890abcdef12  # blob (file1.txt)
        // cdef4567890abcdef1234567890abcdef1234   # blob (file2.txt)
        // ghi7894567890abcdef1234567890abcdef56   # blob (file3.txt)
        // 789abc1234567890abcdef1234567890abc123  # tree (subdir)
    // 12/
    //      34567890abcdef1234567890def456           # tree (root directory)
#[derive(Debug, Clone)]
pub struct Commit {
    id: Uuid,
    parent_id: Uuid,
    message: String,
    created_at: NaiveDateTime,
    branch_id: String,
}

impl PartialEq for Commit {
    fn eq(&self, other: &Self) -> bool {
        self.id.to_string() == other.id.to_string()
            && self.parent_id.to_string() == other.parent_id.to_string()
            && self.created_at == other.created_at
            && self.message == other.message
            && self.branch_id == other.branch_id
    }
}

impl Commit {
    pub fn new(
        id: Uuid,
        parent_id: Uuid,
        message: String,
        created_at: NaiveDateTime,
        branch_id: String,
    ) -> Self {
        Self {
            id,
            parent_id,
            message,
            created_at,
            branch_id,
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
}
