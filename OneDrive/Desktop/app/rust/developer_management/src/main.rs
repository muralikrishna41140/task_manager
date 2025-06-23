mod developer;
mod traits;
mod manager;

use developer::Role;
use manager::DeveloperManager;
use std::io::{self, Write};
use uuid::Uuid;

fn main() {
    let mut manager = DeveloperManager::new();

    loop {
        println!("\n📋 Menu: create | read | update | delete | exit");
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let cmd = input.trim();

        match cmd {
            "create" => {
                print!("Enter name: ");
                io::stdout().flush().unwrap();
                let mut name = String::new();
                io::stdin().read_line(&mut name).unwrap();
                manager.create(name.trim().to_string());
            }

            "read" => {
                manager.read_all();
            }

            "update" => {
                print!("Enter ID: ");
                io::stdout().flush().unwrap();
                let mut id_str = String::new();
                io::stdin().read_line(&mut id_str).unwrap();

                let id = match Uuid::parse_str(id_str.trim()) {
                    Ok(id) => id,
                    Err(_) => {
                        println!("❌ Invalid UUID format.");
                        continue;
                    }
                };

                print!("New name (leave blank to skip): ");
                let mut name = String::new();
                io::stdin().read_line(&mut name).unwrap();
                let name = if name.trim().is_empty() {
                    None
                } else {
                    Some(name.trim().to_string())
                };

                print!("New role (Frontend/Backend/Fullstack/DevOps): ");
                let mut role_input = String::new();
                io::stdin().read_line(&mut role_input).unwrap();
                let role = match role_input.trim() {
                    "Frontend" => Some(Role::Frontend),
                    "Backend" => Some(Role::Backend),
                    "Fullstack" => Some(Role::Fullstack),
                    "DevOps" => Some(Role::DevOps),
                    _ => None,
                };

                print!("Comma-separated new skills (leave blank to skip): ");
                let mut skills_input = String::new();
                io::stdin().read_line(&mut skills_input).unwrap();
                let skills = if skills_input.trim().is_empty() {
                    None
                } else {
                    Some(
                        skills_input
                            .trim()
                            .split(',')
                            .map(|s| s.trim().to_string())
                            .collect(),
                    )
                };

                manager.update(id, name, role, skills);
            }

            "delete" => {
                print!("Enter ID to delete: ");
                io::stdout().flush().unwrap();
                let mut id_str = String::new();
                io::stdin().read_line(&mut id_str).unwrap();
                let id = match Uuid::parse_str(id_str.trim()) {
                    Ok(id) => id,
                    Err(_) => {
                        println!("❌ Invalid UUID.");
                        continue;
                    }
                };
                manager.delete(id);
            }

            "exit" => break,
            _ => println!("❓ Unknown command."),
        }
    }
}
