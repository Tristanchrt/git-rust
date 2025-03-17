use chrono::NaiveDateTime;

pub struct BranchToCreate {
    name: String
}

impl BranchToCreate {

    pub fn new(name: String) -> BranchToCreate {
        Self {
            name
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn create(&self) -> Branch {
        Branch::new(self.name.clone(), Self::now())
    }

    fn now() -> NaiveDateTime {
        NaiveDateTime::parse_from_str(
            &chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            "%Y-%m-%d %H:%M:%S"
        ).unwrap()
    }
}

#[derive(Debug, Clone)]
pub struct Branch {
    name: String,
    created_at: NaiveDateTime
}

impl Branch {
    pub fn new(name: String, created_at: NaiveDateTime) -> Branch {
        Self {
            name,
            created_at
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn created_at(&self) -> NaiveDateTime {
        self.created_at
    }
}

impl PartialEq for Branch {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.created_at == self.created_at
    }
}