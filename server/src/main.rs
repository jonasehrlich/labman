use clap::{Parser, Subcommand};
use dotenvy::dotenv;
use labman_server::{cli, core};
use std::env;

#[derive(Parser)]
#[command(version, about, long_about = None)]
/// Server to manage lab setups, reservations and access
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new user
    CreateUser {
        /// Name of the user
        #[arg(long, short = 'n')]
        name: String,
        /// Role of the user
        #[arg(value_enum, long, short = 'r', ignore_case = true)]
        role: core::models::UserRole,
    },

    /// List the available users
    ListUsers {
        /// Minimum role of the users to list
        #[arg(value_enum, default_value = "reporter", ignore_case = true)]
        min_role: core::models::UserRole,
    },

    /// Delete a user
    DeleteUser {
        /// Name of the user to delete
        #[arg(long, short = 'n')]
        name: String,
    },

    /// Run the server
    Run {
        /// Host to bind the server to
        #[arg(long, default_value = "localhost")]
        host: String,
        /// Port to bind the server to
        #[arg(long, short, default_value_t = 8000, value_parser = clap::value_parser!(u16).range(1024..=65535))]
        port: u16,
    },
}

fn main() {
    let args = Cli::parse();

    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let mut labman = match core::Labman::new(&database_url) {
        Ok(labman) => labman,
        Err(e) => {
            eprintln!("Failed to initialize Labman: {}", e);
            std::process::exit(1);
        }
    };

    match &args.command {
        Commands::CreateUser { name, role } => {
            cli::create_user(&mut labman, name, role);
        }
        Commands::ListUsers { min_role } => {
            cli::list_users(&mut labman, min_role);
        }
        Commands::DeleteUser { name } => {
            cli::delete_user(&mut labman, name);
        }
        Commands::Run { host: _, port: _ } => {
            todo!("run the server")
        }
    }
}
