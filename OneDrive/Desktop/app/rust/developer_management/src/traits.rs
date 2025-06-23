use crate::developer::{Developer, Role};

pub trait DisplayInfo {
    fn display(&self);
}

impl DisplayInfo for Developer {
    fn display(&self) {
        println!("\n--- Developer Info ---");
        println!("ID: {}", self.id);
        println!("Name: {}", self.name);
        println!("Role: {:?}", self.role);
        println!("Skills: {:?}", self.skills);

        match self.role {
            Role::Frontend => println!("💻 UI Specialist"),
            Role::Backend => println!("🗄️ API Expert"),
            Role::Fullstack => println!("🧠 Jack of All Trades"),
            Role::DevOps => println!("⚙️ Pipeline Guru"),
            Role::Unknown => println!("❓ Role not defined"),
        }
    }
}
