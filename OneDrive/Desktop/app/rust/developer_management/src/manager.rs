use crate::developer::{Developer, Role};
use crate::traits::DisplayInfo;
use uuid::Uuid;

pub struct DeveloperManager {
    pub developers: Vec<Developer>,
}

impl DeveloperManager {
    pub fn new() -> Self {
        Self {
            developers: Vec::new(),
        }
    }

    pub fn create(&mut self, name: String) {
        let dev = Developer::new(name);
        self.developers.push(dev);
        println!("✅ Developer added.");
    }

    pub fn read_all(&self) {
        for dev in &self.developers {
            dev.display();
        }
    }

    pub fn read_by_skill(&self, skill: &str) {
        for dev in &self.developers {
            if dev.skills.iter().any(|s| s.eq_ignore_ascii_case(skill)) {
                dev.display();
            }
        }
    }

    pub fn update(
        &mut self,
        id: Uuid,
        new_name: Option<String>,
        new_role: Option<Role>,
        new_skills: Option<Vec<String>>,
    ) {
        if let Some(dev) = self.developers.iter_mut().find(|d| d.id == id) {
            if let Some(name) = new_name {
                dev.name = name;
            }
            if let Some(role) = new_role {
                dev.role = role;
            }
            if let Some(skills) = new_skills {
                dev.skills = skills;
            }
            println!("🔄 Developer updated.");
        } else {
            println!("❌ Developer not found.");
        }
    }

    pub fn delete(&mut self, id: Uuid) {
        let original_len = self.developers.len();
        self.developers.retain(|d| d.id != id);
        if self.developers.len() < original_len {
            println!("🗑️ Developer deleted.");
        } else {
            println!("❌ Developer ID not found.");
        }
    }
}
