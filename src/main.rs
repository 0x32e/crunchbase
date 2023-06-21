use std::env;
use dotenv::dotenv;
use clap::{Subcommand, command, Args, Parser};

mod api;
mod models;

#[derive(Parser)]
#[command(author, version)]
#[command(about = "crunchbase - a simple CLI to fetch funding information from Crunchbase", long_about = "crunchbase is a super fancy CLI (kidding)")]

struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Query(Query),
    Import(Import),
    Inquire(Inquire),
}

#[derive(Args)]
struct Query {

    #[arg(short='i', long="industry")]
    industry: Option<String>,

    #[arg(short='d', long="days")]
    days: Option<i32>,

    #[arg(short='c', long="currency")]
    currency: Option<String>,

    #[arg(short='f', long="funding_type")]
    funding_type: Option<String>,

    #[arg(short='e', long="description")]
    description: Option<String>,

    #[arg(short='l', long="limit")]
    limit: Option<i64>,
}

#[derive(Args)]
struct Import {
    #[arg(short='f', long="filename")]
    filename: Option<String>
}

#[derive(Args)]
struct Inquire {}

#[tokio::main]
async fn main() {

    dotenv().ok();

    let cli: Cli = Cli::parse();
    let cb_postgres_uri = env::var("CB_POSTGRES_URI").expect("CB_POSTGRES_URI must be set");

    let postgres = tokio_postgres::connect(&cb_postgres_uri, tokio_postgres::NoTls).await;

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
        Some(Commands::Query(args)) => {
            match api::query::query(
                 &mut client,
                 args.industry.clone(), 
                 args.days.clone(), 
                 args.limit.clone(), 
                 args.currency.clone(), 
                 args.funding_type.clone(), 
                 args.description.clone()
             ).await {
                 Ok(res) => {
                     api::util::display_table(&res);
                 },
                 Err(e) => {
                     println!("Error {}", e);
                 }
            }
        }
        Some(Commands::Import(args)) => {
            api::import::run_import_prompt(&mut client, args.filename)
                .await
                .unwrap();
        }
        Some(Commands::Inquire(..)) => {
            let res = api::query::run_query_prompt(&mut client)
                .await
                .unwrap();
            api::util::display_table(&res);
        }
        None => {
            println!("Command is missing.");
        }
    }

}
