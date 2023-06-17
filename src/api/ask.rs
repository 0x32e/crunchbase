use llm_chain::{tools::{tools::{BingSearch}, ToolCollection}};
use llm_chain::tools::{Tool};

use llm_chain::{
    agents::self_ask_with_search::{Agent, EarlyStoppingConfig},
    executor,
};

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

pub async fn ask_agent(query: &str) -> Result<String, Box<dyn std::error::Error>> {

    let executor = executor!().unwrap();
    let bing_api_key = std::env::var("BING_API_KEY").unwrap();
    let search_tool = BingSearch::new(bing_api_key);
    let agent = Agent::new(
        executor,
        search_tool,
        EarlyStoppingConfig {
            max_iterations: Some(10),
            max_time_elapsed_seconds: Some(30.0),
        },
    );

    println!("running...");

    let (res, intermediate_steps) = agent
        .run(query)
        .await
        .unwrap();

    println!(
        "Are followup questions needed here: {}",
        agent.build_agent_scratchpad(&intermediate_steps)
    );

    println!(
        "Agent final answer: {}",
        res.return_values.get("output").unwrap()
    );

    Ok(res.return_values.get("output").unwrap())
}