use strum::IntoEnumIterator;

use crate::{Labman, models};

fn print_users<I, E>(users: I) -> Option<E>
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
                return Some(e);
            }
        }
    }
    return None;
}

/// Create a user and print it
pub fn create_user(labman: &mut Labman, name: &String, role: &models::UserRole) {
    let user = labman.user().create(name, role);
    match print_users(std::iter::once(user)) {
        Some(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
        None => {}
    }
}

/// Print users with a minimum role
pub fn list_users(labman: &mut Labman, min_role: &models::UserRole) {
    match labman.user().iter(min_role) {
        Ok(users) => match print_users(users) {
            Some(e) => {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
            None => {}
        },
        Err(e) => {
            println!("Error listing users: {}", e)
        }
    }
}

/// Delete a user by name
pub fn delete_user(labman: &mut Labman, name: &String) {
    match labman.user().delete(name) {
        Some(e) => {
            eprintln!("Error deleting user '{}': {}", name, e);
            std::process::exit(1);
        }
        None => {
            println!("User '{}' deleted successfully.", name);
        }
    }
}
