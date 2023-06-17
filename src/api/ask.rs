use llm_chain::{tools::tools::{BingSearch}, tools::Tool};

pub async fn ask(query: &str) -> Result<(), Box<dyn std::error::Error>> {
    let bing_api_key = std::env::var("BING_API_KEY").unwrap();
    let bing = BingSearch::new(bing_api_key);
    let result = bing
        .invoke_typed(&query.into())
        .await
        .unwrap();
    println!("{}", result.result);
    Ok(())
}