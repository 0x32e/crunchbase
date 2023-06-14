mod funding;
mod api;

use std::fs::OpenOptions;

use dotenv::dotenv;
use clap::{Subcommand, command, Args, Parser};

#[derive(Parser)]
#[command(author, version)]
#[command(about = "stringer - a simple CLI to transform and inspect strings", long_about = "stringer is a super fancy CLI (kidding)
One can use stringer to modify or inspect strings straight from the terminal")]

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

// #[tokio::main]
fn main() {

    dotenv().ok();

    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::Query(args)) => {
            println!("querying...");
            let res = api::query::query(args.industry.clone(), args.days.clone(), args.limit.clone(), args.currency.clone(), args.funding_type.clone(), args.description.clone()).unwrap();
            api::util::display_table(&res);
            println!("querying done...");
            // TODO: Output as a table
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
        None => {
            println!("Command is missing");
        }
    }


}
