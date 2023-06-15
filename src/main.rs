mod funding;
mod api;

use dotenv::dotenv;
use clap::{Subcommand, command, Args, Parser};
use std::thread;

#[derive(Parser)]
#[command(author, version)]
#[command(about = "cbdb - a simple CLI to fetch funding information from Crunchbase", long_about = "cbdb is a super fancy CLI (kidding)")]

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
    days: Option<i8>,

    #[arg(short='c', long="currency")]
    currency: Option<String>,

    #[arg(short='f', long="funding_type")]
    funding_type: Option<String>,

    #[arg(short='e', long="description")]
    description: Option<String>,

    #[arg(short='l', long="limit")]
    limit: Option<i8>,
}

#[derive(Args)]
struct Import {
    file_name: Option<String>,
}

#[derive(Args)]
struct Ask {
    query: Option<String>,
}

fn main() {

    dotenv().ok();

    let cli: Cli = Cli::parse();
    match &cli.command {
        Some(Commands::Query(args)) => {
            let res = api::query::query(args.industry.clone(), args.days.clone(), args.limit.clone(), args.currency.clone(), args.funding_type.clone(), args.description.clone()).unwrap();
            api::util::display_table(&res);
        }
        Some(Commands::Import(args)) => {
            match args.file_name {
                Some(ref filename) => {
                    println!("importing '{}'...", filename);
                    api::import::import(filename).unwrap();
                    println!("importing done");
                }
                None => {
                    println!("Please provide a filename");
                }
            }
        }
        Some(Commands::Ask(args)) => {
            match args.query {
                Some(ref query) => {
                    let query_clone = query.clone();
                    println!("{}", query_clone);
                    // let handle = tokio::spawn(async move {
                    //     let _ = api::ask::ask(&query_clone).await;
                    // });
                    // let _ = handle.await;
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
