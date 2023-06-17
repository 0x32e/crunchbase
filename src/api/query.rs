/*
TODO: 
- Ask an LLM to answer a question against the "fundings" table.
- (Separate) worker program monitors if there's any new csv files downloaded from Crunchbase, and grabs it and process it.
 */

use crate::models::Funding;

use tokio_postgres::Client;

// use llm_chain::chains::map_reduce::Chain;
// use llm_chain::{executor, parameters, prompt, step::Step, Parameters};

// TODO: Able to ask questions for the data in a thread
// TODO: Handle subcommand
// TODO: Median & Average funding amount
// TODO: Use diesel crate
// TODO: Download the latest csv files stored somewhere to update the db before starting the query
// TODO: Finish handling the rest of the filters
// TODO: Add a connector to a vector db and index all the company descriptions or other text data and store them as vectors.
// TODO: Visualize the data (e.g., Group by industries)
// TODO: Input my business ideas and the AI will give me the list of companies I should do research on
// TODO: Business model analyzer
// TODO: Use the "id" field from crunchbase data for easy comparison later
// TODO: News Reporter - AI will automatically crawl the web and fetch some news articles for the companies
// TODO: Company information analyzer (e.g., founders, techstack, etc.)
// TODO: More Error Handlings
pub async fn query(
    client: &mut Client,
    industry: Option<String>, 
    days: Option<i32>,
    limit: Option<i64>,
    currency: Option<String>,
    _funding_type: Option<String>,
    _description: Option<String>,
) -> Result<Vec<Funding>, Box<dyn std::error::Error>> {

    println!("querying...");

    let mut fundings: Vec<Funding> = vec![];

    let industry = industry
        .map_or("".to_owned(), |i|
            format!("and organization_industries LIKE CONCAT('%', '{}'::text, '%')", i).to_owned()
        );

    let res = client.query(&format!("
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
        TO_DATE(announced_date, 'YYYY-MM-DD') >= CURRENT_DATE - make_interval(days := $1) and
        money_raised_currency = $2
        {}
    ORDER BY TO_DATE(announced_date, 'YYYY-MM-DD') DESC
    LIMIT $3", industry), &[&days.unwrap(), &currency.unwrap(), &limit.unwrap()]).await;

    match res {
        Ok(rows) => {
            for row in rows {
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
                fundings.push(funding);
            }
        },
        Err(e) => {
            println!("Error: {}", e);
        }
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

pub struct FundingCount {
    pub industry: String,
    pub count: i64,
}

pub async fn get_funding_count_by_industry(client: &mut Client, last_days: i32, raised_currency: &str) -> Result<Vec<FundingCount>, Box<dyn std::error::Error>> {
    let mut funding_counts: Vec<FundingCount> = vec![];
    let res = client.query("
    SELECT 
        industry, 
        COUNT(*) as funding_count
    FROM (
        SELECT unnest(string_to_array(organization_industries, ', ')) as industry,
            announced_date::date as funding_date,
            money_raised_currency
        FROM fundings
    ) as subquery
    WHERE 
        funding_date >= CURRENT_DATE - make_interval(days := $1) and 
        money_raised_currency = $2
    GROUP BY industry
    HAVING count(*) > 10
    ORDER BY funding_count DESC", &[&last_days, &raised_currency]).await;
    
    match res {
        Ok(rows) => {
            for row in rows {
                funding_counts.push(FundingCount{ industry: row.get(0), count: row.get(1)});
            }
        },
        Err(e) => {
            println!("Error: {}", e);
        }
    }

    Ok(funding_counts)

    // match res {
    //     Ok(rows) => {
    //         for row in rows {
    //             let funding = Funding {
    //                 transaction_name: row.get(0),
    //                 transaction_url: row.get(1),
    //                 organization_name: row.get(2),
    //                 organization_description: row.get(3),
    //                 funding_type: row.get(4),
    //                 money_raised: row.get(5),
    //                 money_raised_currency: row.get(6),
    //                 money_raised_in_usd: row.get(7),
    //                 announced_date: row.get(8),
    //                 number_of_investors: row.get(9),
    //                 number_of_funding_rounds: row.get(10),
    //                 organization_industries: row.get(11),
    //                 organization_location: row.get(12),
    //                 organization_website: row.get(13),
    //             };
    //             fundings.push(funding);
    //         }
    //     },
    //     Err(e) => {
    //         println!("Error: {}", e);
    //     }
    // }

}