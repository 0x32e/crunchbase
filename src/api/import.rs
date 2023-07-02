use inquire::Text;
use std::fs::File;
use std::io::Read;
use std::fs::read_dir;

use crate::models::Funding;
use tokio_postgres::Client;

const IMPORT_PROMPT: &str = "Enter the filename to import data from (the file has to be under \"./data\" directory):";

pub async fn run_import_prompt(
    client: &mut Client, 
    filename: Option<String>,
    all: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    if all {
        import_all(client).await
    } else {
        let filename_str = match filename {
            Some(f) => f,
            None => Text::new(IMPORT_PROMPT)
                .prompt()
                .unwrap(),
        };
        import(client, &filename_str).await
    }
}

pub async fn import_all(client: &mut Client) -> Result<(), Box<dyn std::error::Error>> {
    // Read all the files inside /data directory, and import the file serially
    let paths = read_dir("./data").unwrap();
    for path in paths {
        let filename = path.unwrap().file_name().to_str().unwrap().to_string();
        let _ = import(client, &filename).await;
    }
    Ok(())
}

pub async fn import(client: &mut Client, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Importing {filename}...");

    // TODO: Check if all the columns are present
    // TODO: Use buffer to handle large csv files (e.g., line by line) 
    let mut file = File::open(format!("data/{}", filename))?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;

    let mut reader = csv::Reader::from_reader(data.as_bytes());

    // let mut duplicate_fundings: Vec<String> = Vec::new();
    let mut duplicate_count = 0;
    let mut inserted_count = 0;

    for result in reader.deserialize() {
        let mut record: Funding = result?;
        let exist = record_exists(client, &record).await;
        if exist {
            // duplicate_fundings.push(record.transaction_name.unwrap());
            duplicate_count += 1;
        } else {
            insert_funding_record(client, &mut record).await?;
            inserted_count += 1;
        }
    }

    println!("Inserted: {}", inserted_count);
    println!("Already exists: {}", duplicate_count);

    Ok(())
}

async fn insert_funding_record(client: &mut Client, funding: &mut Funding) -> Result<(), Box<dyn std::error::Error>> {
    client.execute(
        "INSERT INTO fundings (
            transaction_name, 
            transaction_url, 
            organization_name, 
            organization_description,
            funding_type,
            money_raised,
            money_raised_currency,
            money_raised_in_usd,
            announced_date,
            number_of_investors,
            number_of_funding_rounds,
            organization_industries,
            organization_location,
            organization_website
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)",
        &[
            &funding.transaction_name, 
            &funding.transaction_url, 
            &funding.organization_name,
            &funding.organization_description,
            &funding.funding_type,
            &funding.money_raised,
            &funding.money_raised_currency,
            &funding.money_raised_in_usd,
            &funding.announced_date,
            &funding.number_of_investors,
            &funding.number_of_funding_rounds,
            &funding.organization_industries,
            &funding.organization_location,
            &funding.organization_website,
        ],
    ).await.unwrap();

    Ok(())
}

async fn record_exists(client: &mut Client, funding: &Funding) -> bool {
    let row = client.query_one("
        SELECT
            count(1)
        FROM fundings
        WHERE 
            transaction_name = $1 and
            transaction_url = $2 and
            announced_date = $3
        ",
        &[
            &funding.transaction_name, 
            &funding.transaction_url, 
            &funding.announced_date
        ],
    ).await.unwrap();
    let count: i64 = row.get(0);

    count > 0
}

