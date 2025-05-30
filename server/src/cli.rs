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
    Ok(())
}

/// Create a user and print it
pub async fn create_user(labman: &Labman, name: &str, role: &models::UserRole) {
    let user = labman.user().create(name, role).await;
    if let Err(e) = print_users(std::iter::once(user)) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

/// Print users with a minimum role
pub async fn list_users(labman: &Labman, min_role: &models::UserRole) {
    match labman.user().iter(min_role).await {
        Ok(users) => {
            if let Err(e) = print_users(users) {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
        Err(e) => {
            println!("Error listing users: {}", e)
        }
    }
}

/// Delete a user by name
pub async fn delete_user(labman: &Labman, name: &String) {
    match labman.user().delete(name).await {
        Err(e) => {
            eprintln!("Error deleting user '{}': {}", name, e);
            std::process::exit(1);
        }
        Ok(()) => {
            println!("User '{}' deleted successfully.", name);
        }
    }
}
