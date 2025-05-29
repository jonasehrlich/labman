use crate::core::{Labman, models};
use strum::IntoEnumIterator;

fn print_users<I, E>(users: I) -> Result<(), E>
where
    I: IntoIterator<Item = Result<models::User, E>>,
    E: std::fmt::Display,
{
    // Find the maximum width of the UserRole variants
    let max_role_width = models::UserRole::iter()
        .map(|role| format!("{:?}", role).len() + 2)
        .max()
        .unwrap_or(10);

    println!("{:<20} | {:<max_role_width$}", "Name", "Role");
    println!("{:-<20}-+-{:-<max_role_width$}", "", "");

    for user in users {
        match user {
            Ok(user) => {
                println!(
                    "{:<20} | {:<max_role_width$}",
                    user.name,
                    format!("{:?}", user.role),
                );
            }
            Err(e) => {
                return Err(e);
            }
        }
    }
    return Ok(());
}

/// Create a user and print it
pub fn create_user(labman: &mut Labman, name: &String, role: &models::UserRole) {
    let user = labman.user().create(name, role);
    match print_users(std::iter::once(user)) {
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
        Ok(()) => {}
    }
}

/// Print users with a minimum role
pub fn list_users(labman: &mut Labman, min_role: &models::UserRole) {
    match labman.user().iter(min_role) {
        Ok(users) => match print_users(users) {
            Err(e) => {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
            Ok(()) => {}
        },
        Err(e) => {
            println!("Error listing users: {}", e)
        }
    }
}

/// Delete a user by name
pub fn delete_user(labman: &mut Labman, name: &String) {
    match labman.user().delete(name) {
        Err(e) => {
            eprintln!("Error deleting user '{}': {}", name, e);
            std::process::exit(1);
        }
        Ok(()) => {
            println!("User '{}' deleted successfully.", name);
        }
    }
}
