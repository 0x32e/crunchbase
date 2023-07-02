use dotenv::dotenv;
use openai_api_rs::v1::chat_completion::{ChatCompletionRequest, self, MessageRole};
use std::env;
use crate::models::{Funding, Fundings};

use inquire::{Select, Text, CustomUserError};
use tokio_postgres::Client;
use openai_api_rs::v1::api::Client as OpenAIClient;

const INDUSTRY_PROMPT: &str = "Industry:";
// TODO: To be dynamically fetched
const INDUSTRY_OPTIONS: &[&str] = &[
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

const DAYS_PROMPT: &str = "Days (e.g., last X days):";
const DAYS_OPTIONS: &[&str] = &["5", "10", "15", "20", "30", "60"];

const CURRENCY_PROMPT: &str = "Currency (e.g., USD):";
const CURRENCY_OPTIONS: &[&str] = &["USD", "EUR", "CAD", "INR", "AUD", "GBP"];

const QUESTION_PROMPT: &str = "Enter a question:";

const SYSTEM_PROMPT: &str = "You are a business anaylyst at a VC firm with access to startup funding data. You try your best to answer user's question.";

const FOLLOWUP_PROMPT: &str = "Ask any follow-up questions, or \"n\" to stop:";

struct ChatMessage {
    content: String,
    role: chat_completion::MessageRole, 
}

impl Into<chat_completion::ChatCompletionMessage> for ChatMessage {
    fn into(self) -> chat_completion::ChatCompletionMessage {
        chat_completion::ChatCompletionMessage { 
            role: self.role, 
            content: Some(self.content), 
            name: None, 
            function_call: None 
        }
    }
}

impl Clone for ChatMessage {
    fn clone(&self) -> Self {
        let role = match self.role {
            MessageRole::user => MessageRole::user,
            MessageRole::system => MessageRole::system,
            MessageRole::assistant => MessageRole::assistant,
        };
        ChatMessage { content: self.content.clone(), role }
    }
}

pub async fn run_query_prompt(
    client: &mut Client
) -> Result<(), Box<dyn std::error::Error>> {

    dotenv().ok();
    
    let industry = Select::new(INDUSTRY_PROMPT, INDUSTRY_OPTIONS.to_vec())
        .prompt()
        .unwrap()
        .to_owned();
    
    let days = Select::new(DAYS_PROMPT, DAYS_OPTIONS.to_vec())
        .prompt()
        .unwrap()
        .to_owned()
        .parse::<i32>()
        .unwrap();

    let currency = Select::new(CURRENCY_PROMPT, CURRENCY_OPTIONS.to_vec())
        .prompt()
        .unwrap()
        .to_owned();
    
    let res = query(
        client, 
        Some(industry.clone()), 
        Some(days), 
        Some(currency), 
        None, 
        None
    ).await.unwrap();

    let question = Text::new(QUESTION_PROMPT)
        .prompt()
        .unwrap();

    let fundings = Fundings(res);
    let context = fundings.to_csv_string();

    let mut messages = vec![
        ChatMessage {
            role: MessageRole::system,
            content: format!("{}\nThe domain of the data points is {}\nContext:\n{}", SYSTEM_PROMPT, industry.clone(), context),
        },
        ChatMessage {
            role: chat_completion::MessageRole::user,
            content: format!("Question: {}", question),
        },
    ];
    
    // TODO: Replace the following with a smart agent
    loop {
        let answer = match answer(&messages).await {
            Ok(a) => {
                println!("Answer: {}", a);
                a
            },
            Err(e) => {
                println!("Error occurrerd: {}", e);
                break;
            }
        };

        messages.push(ChatMessage{
            role: MessageRole::assistant,
            content: answer,
        });

        let followup_message = Text::new(FOLLOWUP_PROMPT)
            .prompt()
            .unwrap();
        if followup_message == "n" {
            break;
        } else {
            messages.push(ChatMessage{
                role: MessageRole::user,
                content: followup_message,
            });
        }
    }

    Ok(())
}

async fn answer(
    messages: &[ChatMessage]
) -> Result<String, Box<dyn std::error::Error>> {
    let client = OpenAIClient::new(env::var("OPENAI_API_KEY")
        .unwrap()
        .to_string()
    );
    
    let req = ChatCompletionRequest {
        model: chat_completion::GPT4.to_string(),
        messages: messages.iter().map(|m| m.clone().into()).collect(),
        functions: None,
        function_call: None,
    };

    let answer = match client.chat_completion(req).await {
        Ok(a) => a.choices[0].message.content.clone().unwrap(),
        Err(e) => {
            return Err(CustomUserError::from(e));
        },
    };

    Ok(answer)
}

async fn query(
    client: &mut Client,
    industry: Option<String>, 
    days: Option<i32>,
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
    ORDER BY TO_DATE(announced_date, 'YYYY-MM-DD') DESC LIMIT 100", industry), &[&days.unwrap(), &currency.unwrap()]).await;
    
    // TODO: Rewrite this with combinators
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
            return Err(CustomUserError::from(e));
        }
    }

    Ok(fundings)
}

