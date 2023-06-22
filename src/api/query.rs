/*
TODO: 
- Ask an LLM to answer a question against the "fundings" table.
- (Separate) worker program monitors if there's any new csv files downloaded from Crunchbase, and grabs it and process it
 */

use dotenv::dotenv;
use std::env;
use crate::models::{Funding, Fundings};

use inquire::{Select, Text};
use tokio_postgres::Client;
use openai_api_rs::v1::api::Client as OpenAIClient;
use openai_api_rs::v1::completion::{self, CompletionRequest};

pub async fn run_query_prompt(
    client: &mut Client
) -> Result<(), Box<dyn std::error::Error>> {

    dotenv().ok();
    
    let industry_options = vec![
        "Crypto",
        "Artificial Intelligence",
        "Financial Services",
        "Wellness",
        "Enterprise",
        "Cloud Computing",
        "Health Care",
        "Blockchain",
        "Energy",
        "Software",
        "Medical",
        "Bio",
        "E-Commerce",
        "Food and Beverage",
        "Manufacturing",
        "Education",
        "Robotics",
        "Human Resources",
        "Real Estate"
    ];

    let industry = Select::new("Select industry:", industry_options)
        .prompt()
        .unwrap()
        .to_owned();
    
    let days = Select::new("Days (e.g., last X days):", vec!["5", "10", "15", "20", "30", "60"])
        .prompt()
        .unwrap()
        .to_owned()
        .parse::<i32>()
        .unwrap();

    let currency = Select::new("Currency (e.g., USD):", vec!["USD", "JPY", "CAD"])
        .prompt()
        .unwrap()
        .to_owned();
    
    let limit = Select::new(
        "How many max results do you want? (i.e., limit)", 
        vec!["10", "20", "30", "60"]
    )
        .prompt()
        .unwrap()
        .parse::<i64>()
        .unwrap();

    let res = query(
        client, 
        Some(industry), 
        Some(days), 
        Some(limit), 
        Some(currency), 
        None, 
        None
    ).await.unwrap();

    let question = Text::new("Enter a question if you have any:")
        .prompt_skippable()
        .ok()
        .unwrap()
        .unwrap();

    let fundings = Fundings(res);
    let context = fundings.to_csv_string();

    let answer = answer(&context, &question).await.unwrap();
    println!("answer: {}", answer);

    Ok(())
}

async fn answer(
    context: &str, 
    question: &str
) -> Result<String, Box<dyn std::error::Error>> {
    let client = OpenAIClient::new(env::var("OPENAI_API_KEY").unwrap().to_string());
    let prompt = format!("You have the following data: {}
    ==========
    Based on these data, answer the following question: {}:\n", context, question);
    // println!("Prompt: {}", prompt);
    let req = CompletionRequest {
        model: completion::GPT3_TEXT_DAVINCI_003.to_string(),
        prompt: Some(String::from(prompt)),
        suffix: None,
        max_tokens: Some(2000),
        temperature: Some(0.0),
        top_p: Some(1.0),
        n: None,
        stream: None,
        logprobs: None,
        echo: None,
        stop: Some(vec![String::from(" Human:"), String::from(" AI:")]),
        presence_penalty: Some(0.6),
        frequency_penalty: Some(0.0),
        best_of: None,
        logit_bias: None,
        user: None,
    };

    let answer = match client.completion(req).await {
        Ok(a) => a.choices[0].text.clone(),
        Err(e) => {
            String::from(format!("Failed: {}", e))
        },
    };
    Ok(answer)
}

pub async fn query(
    client: &mut Client,
    industry: Option<String>, 
    days: Option<i32>,
    limit: Option<i64>,
    currency: Option<String>,
    _funding_type: Option<String>,
    _description: Option<String>,
) -> Result<Vec<Funding>, Box<dyn std::error::Error>> {
    
    // TODO: the data should be pulled from a central db rather than csv files.
    // Crunchbase API sucks and I don't want to rely on it.

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

    Ok(fundings)
}

