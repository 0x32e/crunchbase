use dotenv::dotenv;
use std::env;
use postgres::{Client, NoTls};
use std::fs::File;
use std::io::Read;
use crate::funding::Funding;

// TODO: Add a flag (e.g., --override -o) to indicate whether to update the existing rows with the new ones from the csv file
pub fn import(filename: &String) -> Result<(), Box<dyn std::error::Error>> {

    dotenv().ok();

    let cb_postgres_uri = env::var("CB_POSTGRES_URI").expect("CB_POSTGRES_URI must be set");
    let mut client = Client::connect(cb_postgres_uri.as_str(), NoTls)?;

    // TODO: I want to check if all the columns are present
    let mut file = File::open(format!("data/{}", filename))?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;

    let mut reader = csv::Reader::from_reader(data.as_bytes());

    // let mut duplicate_fundings: Vec<String> = Vec::new();
    let mut duplicate_count = 0;
    let mut inserted_count = 0;

    for result in reader.deserialize() {
        let mut record: Funding = result?;
        let exist = record_exists(&mut client, &record);
        if exist {
            // duplicate_fundings.push(record.transaction_name.unwrap());
            duplicate_count += 1;
        } else {
            insert_funding_record(&mut client, &mut record)?;
            inserted_count += 1;
        }
    }

    println!("inserted: {}", inserted_count);
    println!("already exists: {}", duplicate_count);

    Ok(())
}

fn insert_funding_record(client: &mut Client, funding: &mut Funding) -> Result<(), Box<dyn std::error::Error>> {
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
    )?;

    Ok(())
}

fn record_exists(client: &mut Client, funding: &Funding) -> bool {
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
    ).unwrap();
    let count: i64 = row.get(0);

    count > 0
}
