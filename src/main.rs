use std::env;
use dotenv::dotenv;
use clap::{Subcommand, command, Args, Parser};

mod api;
mod models;

#[derive(Parser)]
#[command(author, version)]
#[command(
    about = "crunchbase - a simple CLI to fetch funding information from Crunchbase", 
)]

struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Query(Query),
    Import(Import),
}

#[derive(Args)]
struct Query {}

#[derive(Args)]
struct Import {
    #[arg(short='f', long="filename")]
    filename: Option<String>,
    
    #[arg(short='a', long="all", default_value_t=false)]
    all: bool
}

#[tokio::main]
async fn main() {

    dotenv().ok();

    let cli: Cli = Cli::parse();
    let postgres_uri = env::var("POSTGRES_URI").expect("POSTGRES_URI must be set");

    let postgres = tokio_postgres::connect(&postgres_uri, tokio_postgres::NoTls).await;

    if let Err(e) = postgres {
        println!("Failed to connect to a Postgres db instance: {}", e);
        return;
    }

    let (mut client, connection) = postgres.unwrap();
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            panic!("connection error: {}", e);
        }
    });

    match cli.command {
        Some(Commands::Query(..)) => {
            api::query::run_query_prompt(&mut client)
                .await
                .unwrap();
        }
        Some(Commands::Import(args)) => {
            api::import::run_import_prompt(&mut client, args.filename, args.all)
                .await
                .unwrap();
        }
        None => {
            println!("Command is missing.");
        }
    }
}
