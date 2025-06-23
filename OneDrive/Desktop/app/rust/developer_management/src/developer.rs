use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum Role {
    Frontend,
    Backend,
    Fullstack,
    DevOps,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct Developer {
    pub id: Uuid,
    pub name: String,
    pub role: Role,
    pub skills: Vec<String>,
}

impl Developer {
    pub fn new(name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            role: Role::Unknown,
            skills: vec![],
        }
    }
}
