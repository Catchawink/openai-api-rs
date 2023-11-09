use openai_api_rs::v1::api::Client;
use openai_api_rs::v1::chat_completion::{self, ChatCompletionRequest};
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new(env::var("OPENAI_API_KEY").unwrap().to_string());

    let req = ChatCompletionRequest::new(
        chat_completion::GPT4.to_string(),
        vec![chat_completion::ChatCompletionMessage {
            role: Some(chat_completion::MessageRole::user),
            content: String::from("What is Bitcoin?"),
            name: None,
            function_call: None,
        }],
    );

    let result = client.chat_completion(req)?;
    println!("{:?}", result.choices[0].message.clone().unwrap().content);

    Ok(())
}

// OPENAI_API_KEY=xxxx cargo run --package openai-api-rs --example chat_completion
