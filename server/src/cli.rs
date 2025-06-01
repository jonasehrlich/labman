use crate::core::{Labman, entity};
use sea_orm::Iterable;

fn print_users<I>(users: I)
where
    I: IntoIterator<Item = entity::user::Model>,
{
    // Find the maximum width of the UserRole variants
    let max_role_width = entity::user::UserRole::iter()
        .map(|role| format!("{:?}", role).len() + 2)
        .max()
        .unwrap_or(10);

    println!("{:<20} | {:<max_role_width$}", "Name", "Role");
    println!("{:-<20}-+-{:-<max_role_width$}", "", "");

    for user in users {
        println!(
            "{:<20} | {:<max_role_width$}",
            user.name,
            format!("{:?}", user.role),
        )
    }
}

/// Create a user and print it
pub async fn create_user(labman: &Labman, name: &str, role: &entity::user::UserRole) {
    match labman.user().create(name, role).await {
        Ok(user) => {
            print_users(std::iter::once(user));
        }
        Err(e) => {
            eprintln!("Error creating user '{}': {}", name, e);
            std::process::exit(1);
        }
    }
}

/// Print users with a minimum role
pub async fn list_users(labman: &Labman, min_role: &entity::user::UserRole) {
    match labman.user().list(min_role).await {
        Ok(users) => {
            print_users(users);
        }
        Err(e) => {
            println!("Error listing users: {}", e)
        }
    }
}

/// Delete a user by name
pub async fn delete_user(labman: &Labman, name: &String) {
    let user_manager = labman.user();

    match user_manager.get_by_name(name).await {
        Err(e) => {
            eprintln!("Error getting user '{}': {}", name, e);
            std::process::exit(1);
        }
        Ok(Some(user)) => {
            user_manager.delete(user.id).await.unwrap_or_else(|e| {
                eprintln!("Error deleting user '{}': {}", name, e);
                std::process::exit(1);
            });
            println!("User '{}' deleted successfully.", name);
        }
        Ok(None) => {
            eprintln!("User '{}' not found.", name);
            std::process::exit(1);
        }
    }
}
