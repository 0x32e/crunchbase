use std::env;
use dotenv::dotenv;
use clap::{Subcommand, command, Args, Parser};

mod api;
mod models;

#[derive(Parser)]
#[command(author, version)]
#[command(about = "crunchbase - a simple CLI to fetch funding information from Crunchbase", long_about = "cbdb is a super fancy CLI (kidding)")]

struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Query(Query),
    Import(Import),
    Ask(Ask),
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
    file_name: Option<String>,
}

#[derive(Args)]
struct Ask {
    query: Option<String>,
}

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
            // match api::query::query(
            //     &mut client,
            //     args.industry.clone(), 
            //     args.days.clone(), 
            //     args.limit.clone(), 
            //     args.currency.clone(), 
            //     args.funding_type.clone(), 
            //     args.description.clone()
            // ).await {
            //     Ok(res) => {
            //         api::util::display_table(&res);
            //     },
            //     Err(e) => {
            //         println!("Error {}", e);
            //     }
            // }
            
            let last_days = 10;
            let raised_currency = "USD";
            match api::query::get_funding_count_by_industry(&mut client, last_days, raised_currency).await {
                Ok(res) => {
                    api::util::display_funding_count(&res, last_days);
                },
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }
        Some(Commands::Import(args)) => {
            match args.file_name {
                Some(ref filename) => {
                    println!("importing '{}'...", filename);
                    api::import::import(&mut client, filename).await.unwrap();
                    println!("importing done");
                }
                None => {
                    println!("Please provide a filename");
                }
            }
        }
        Some(Commands::Ask(args)) => {
            match args.query {
                Some(query) => {
                    match api::ask::ask(&query.clone()).await {
                        Ok(_) => {},
                        Err(e) => {
                            println!("error: {}", e);
                        },
                    }

                    // match api::ask::ask_agent(&query).await {
                    //     Ok(res) => {
                    //         println!("Response: {}", res);
                    //     },
                    //     Err(e) => {
                    //         println!("error: {}", e);
                    //     }
                    // }
                }
                None => {
                    println!("Please provide a query");
                }
            }
        }
        None => {
            println!("Command is missing");
        }
    }

}
