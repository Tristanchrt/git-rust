use chrono::NaiveDateTime;

pub struct BranchToCreate {
    name: String,
    is_current: bool
}

impl BranchToCreate {

    pub fn new(name: String, is_current: bool) -> BranchToCreate {
        Self {
            name,
            is_current
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn is_current(&self) -> bool {
        self.is_current.clone()
    }
    pub fn create(&self) -> Branch {
        Branch::new(self.name.clone(), Self::now(), self.is_current)
    }

    fn now() -> NaiveDateTime {
        NaiveDateTime::parse_from_str(
            &chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            "%Y-%m-%d %H:%M:%S"
        ).unwrap()
    }
}

pub struct Branch {
    name: String,
    created_at: NaiveDateTime,
    is_current: bool
}

impl Branch {
    pub fn new(name: String, created_at: NaiveDateTime, is_current: bool) -> Branch {
        Self {
            name,
            created_at,
            is_current
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn created_at(&self) -> NaiveDateTime {
        self.created_at
    }

    pub fn is_current(&self) -> bool {
        self.is_current.clone()
    }
}

impl PartialEq for Branch {
    fn eq(&self, other: &Self) -> bool {
      self.name == other.name && self.is_current == other.is_current && self.created_at == other.created_at
    }
}