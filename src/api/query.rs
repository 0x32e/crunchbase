/*
TODO: 
- Ask an LLM to answer a question against the "fundings" table.
- 
 */

use std::fs::File;
use std::io::Read;
use std::env;

use crate::funding::Funding;

use postgres::{Client, NoTls};

use llm_chain::chains::map_reduce::Chain;
use llm_chain::{executor, parameters, prompt, step::Step, Parameters};

pub fn query(
    industry: Option<String>, 
    days: Option<i8>,
    limit: Option<i8>,
    currency: Option<String>,
    funding_type: Option<String>,
    description: Option<String>,
) -> Result<Vec<Funding>, Box<dyn std::error::Error>> {

    let cb_postgres_uri = env::var("CB_POSTGRES_URI").expect("CB_POSTGRES_URI must be set");
    let mut client = Client::connect(cb_postgres_uri.as_str(), NoTls)?;

    let mut fundings: Vec<Funding> = vec![];

    for row in client.query(&format!("
        SELECT 
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
        FROM fundings
        WHERE 
            TO_DATE(announced_date, 'YYYY-MM-DD') >= CURRENT_DATE - INTERVAL '{} days'
        ORDER BY created DESC
        LIMIT {}", days.unwrap(), limit.unwrap()), &[])? {
        let funding = Funding {
            transaction_name: row.get(0),
            transaction_url: row.get(1),
            organization_name: row.get(2),
            organization_description: row.get(3),
            funding_type: row.get(4),
            money_raised: row.get(5),
            money_raised_currency: row.get(6),
            money_raised_in_usd: row.get(7),
            announced_date: row.get(8),
            number_of_investors: row.get(9),
            number_of_funding_rounds: row.get(10),
            organization_industries: row.get(11),
            organization_location: row.get(12),
            organization_website: row.get(13),
        };
        // println!("{:?}", funding);
        fundings.push(funding);
    }

    // // TODO: I want to check if all the columns are present
    // let mut file = File::open("data/fundings_lifestyle.csv")?;
    // let mut data = String::new();
    // file.read_to_string(&mut data)?;

    // let mut reader = csv::Reader::from_reader(data.as_bytes());

    // let mut duplicate_fundings: Vec<String> = Vec::new();
    // let mut duplicate_count = 0;

    // for result in reader.deserialize() {
    //     let mut record: Funding = result?;
    //     let exist = record_exists(&mut client, &record);
    //     if exist {
    //         duplicate_fundings.push(record.transaction_name.unwrap());
    //         duplicate_count += 1;
    //     } else {
    //         insert_funding_record(&mut client, &mut record)?;
    //     }
    // }

    // println!("Duplicate count: {}", duplicate_count);
    // println!("Duplicate transactions: {:?}", duplicate_fundings);

    // let exec = executor!()?;

    // let step = Step::for_prompt_template(prompt!(
    //     "You are an analytics bot which has access to a database with a table called 'fundings' which looks like the following: {{table_ddl}}",
    //     "Based on this table, answer the following question: {{question}}"
    // ));

    // let mut file = File::open("schema.sql")?;
    // let mut schema = String::new();
    // file.read_to_string(&mut schema)?;

    // let question = "generate a sql to get the fundings that are announced in the last 30 days in the wellness industry. Make sure to return the deals in the US.";
    // let res = step
    //     .run(&parameters!("table_ddl" => schema, "question" => question), &exec)
    //     .await?;

    // println!("{}", res);

    // Ok(())

    // Create a new ChatGPT executor with the default settings
    // let exec = executor!()?;

    // // Create the "map" step to summarize an article into bullet points
    // let map_prompt = Step::for_prompt_template(prompt!(
    //     "You are a bot for summarizing wikipedia articles, you are terse and focus on accuracy",
    //     "Summarize this article into bullet points:\n{{text}}"
    // ));

    // // Create the "reduce" step to combine multiple summaries into one
    // let reduce_prompt = Step::for_prompt_template(prompt!(
    //     "You are a diligent bot that summarizes text",
    //     "Please combine the articles below into one summary as bullet points:\n{{text}}"
    // ));

    // // Create a map-reduce chain with the map and reduce steps
    // let chain = Chain::new(map_prompt, reduce_prompt);

    // // Load the content of the article to be summarized
    // let article = include_str!("../../article_to_summarize.md");

    // // Create a vector with the Parameters object containing the text of the article
    // let docs = vec![parameters!(article)];

    // // Run the chain with the provided documents and an empty Parameters object for the "reduce" step
    // let res = chain.run(docs, Parameters::new(), &exec).await.unwrap();

    // // Print the result to the console
    // println!("{}", res.to_immediate().await?.as_content());
    Ok(fundings)
}
