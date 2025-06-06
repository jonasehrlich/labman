use clap::{Parser, Subcommand};
use dotenvy::dotenv;
use labman_server::{cli, core, web};
use std::env;
use std::sync::Arc;

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
        role: core::entity::user::UserRole,
    },

    /// List the available users
    ListUsers {
        /// Minimum role of the users to list
        #[arg(value_enum, default_value = "reporter", ignore_case = true)]
        min_role: core::entity::user::UserRole,
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
        #[arg(long, default_value_t = 8000, value_parser = clap::value_parser!(u16).range(1024..=65535))]
        port: u16,
    },
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let labman = Arc::new(
        core::Labman::new(&database_url)
            .await
            .unwrap_or_else(|err| {
                eprintln!("Failed to create shared state: {}", err);
                std::process::exit(1);
            }),
    );

    match &args.command {
        Commands::CreateUser { name, role } => {
            cli::create_user(&labman, name, role).await;
        }
        Commands::ListUsers { min_role } => {
            cli::list_users(&labman, min_role).await;
        }
        Commands::DeleteUser { name } => {
            cli::delete_user(&labman, name).await;
        }
        Commands::Run { host, port } => {
            web::App::new(labman.clone())
                .await
                .unwrap_or_else(|err| {
                    eprintln!("Failed to create app: {}", err);
                    std::process::exit(1);
                })
                .serve(host, *port)
                .await
                .unwrap_or_else(|err| {
                    eprintln!("Failed to start server: {}", err);
                    std::process::exit(1);
                });
        }
    }
}
