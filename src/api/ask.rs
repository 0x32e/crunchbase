use llm_chain::{tools::{tools::{BingSearch}}};
use llm_chain::tools::{Tool};

use llm_chain::{
    chains::conversation::Chain,
    agents::self_ask_with_search::{Agent, EarlyStoppingConfig},
    executor,
    prompt,
    parameters,
    step::Step
};

#[allow(dead_code)]
pub async fn run_chatgpt(question: &str) -> Result<(), Box<dyn std::error::Error>> {
    let exec = executor!()?;

    // TODO: Have two LLMs talk to each other

    let mut chain1 = Chain::new(
        prompt!(system: format!("
            You are a helpful personal assistant for responding to the other assistant(\"FRIDAY\")'s messages. 
            Be creative and ask any follow up questions about the topic.

            Your name is 'JARVIS'. 
            You are discussing the topic: 
            \"{}\".
        ", question).as_str()),
    )?;

    let mut chain2 = Chain::new(
        prompt!(system: format!("
            You are a helpful & friendly personal assistant for responding to the other assistant(\"JARVIS\")'s messages. 
            Be creative and ask any follow up questions about the topic.

            Your name is 'FRIDAY'.
            You are discussing the topic: 
            \"{}\"
        ", question).as_str()),
    )?;

    let mut messages: Vec<String>= vec![];

    let mut prev_chain_1_message = "".to_string();
    let mut prev_chain_2_message = "".to_string();

    let mut counter = 0;
    loop {
        if counter > 10 {
            break;
        }

        if counter % 2 == 0 {
            // JARVIS's turn
            let mut step = Step::for_prompt_template(prompt!(system: "State your opinion on the topic based on your knowledge."));
            if !prev_chain_1_message.is_empty() {
                step = Step::for_prompt_template(prompt!(system: prev_chain_2_message.as_str()));
            }
            let res = chain1.send_message(step, &parameters!(), &exec).await?;
            prev_chain_1_message = format!("{}", res.to_immediate().await?);
            messages.push(prev_chain_1_message.clone());
            println!("[JARVIS]: {}", prev_chain_1_message);
        } else {
            // FRIDAY's turn
            let step = Step::for_prompt_template(prompt!(user: prev_chain_1_message.as_str()));
            let res = chain2.send_message(step, &parameters!(), &exec).await?;
            prev_chain_2_message = format!("{}", res.to_immediate().await?);
            messages.push(prev_chain_2_message.clone());
            println!("[FRIDAY]: {}", prev_chain_2_message);
        }

        counter += 1;
    }

    // TODO: Summarize using the messages array

    // Define the conversation steps.
    // let step1 = Step::for_prompt_template(prompt!(user: "Make a personalized greeting for Joe."));
    // let step2 =
    //     Step::for_prompt_template(prompt!(user: "Now, create a personalized greeting for Jane."));
    // let step3 = Step::for_prompt_template(
    //     prompt!(user: "Finally, create a personalized greeting for Alice."),
    // );

    // let step4 = Step::for_prompt_template(prompt!(user: "Remind me who did we just greet."));

    // // Execute the conversation steps.
    // let res1 = chain1.send_message(step1, &parameters!(), &exec).await?;
    // println!("Step 1: {}", res1.to_immediate().await?);

    // let res2 = chain.send_message(step2, &parameters!(), &exec).await?;
    // println!("Step 2: {}", res2.to_immediate().await?);

    // let res3 = chain.send_message(step3, &parameters!(), &exec).await?;
    // println!("Step 3: {}", res3.to_immediate().await?);

    // let res4 = chain.send_message(step4, &parameters!(), &exec).await?;
    // println!("Step 4: {}", res4.to_immediate().await?);

    Ok(())
}

#[allow(dead_code)]
async fn ask(query: &str) -> Result<(), Box<dyn std::error::Error>> {
    let bing_api_key = std::env::var("BING_API_KEY").unwrap();
    let bing = BingSearch::new(bing_api_key);
    let result = bing
        .invoke_typed(&query.into())
        .await
        .unwrap();
    println!("{}", result.result);
    Ok(())
}

#[allow(dead_code)]
async fn ask_agent(query: &str) -> Result<String, Box<dyn std::error::Error>> {

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
