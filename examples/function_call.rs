use openai_api_rs::v1::api::Client;
use openai_api_rs::v1::chat_completion::{self, ChatCompletionRequest, FunctionCallType};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::{env, vec};

fn get_coin_price(coin: &str) -> f64 {
    let coin = coin.to_lowercase();
    match coin.as_str() {
        "btc" | "bitcoin" => 10000.0,
        "eth" | "ethereum" => 1000.0,
        _ => 0.0,
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new(env::var("OPENAI_API_KEY").unwrap().to_string());

    let mut properties = HashMap::new();
    properties.insert(
        "coin".to_string(),
        Box::new(chat_completion::JSONSchemaDefine {
            schema_type: Some(chat_completion::JSONSchemaType::String),
            description: Some("The cryptocurrency to get the price of".to_string()),
            enum_values: None,
            properties: None,
            required: None,
            items: None,
        }),
    );

    let req = ChatCompletionRequest::new(
        chat_completion::GPT3_5_TURBO_0613.to_string(),
        vec![chat_completion::ChatCompletionMessage {
            role: Some(chat_completion::MessageRole::user),
            content: String::from("What is the price of Ethereum?"),
            name: None,
            function_call: None,
        }],
    )
    .functions(vec![chat_completion::Function {
        name: String::from("get_coin_price"),
        description: Some(String::from("Get the price of a cryptocurrency")),
        parameters: chat_completion::FunctionParameters {
            schema_type: chat_completion::JSONSchemaType::Object,
            properties: Some(properties),
            required: Some(vec![String::from("coin")]),
        },
    }])
    .function_call(FunctionCallType::Auto);

    // debug reuqest json
    // let serialized = serde_json::to_string(&req).unwrap();
    // println!("{}", serialized);

    let result = client.chat_completion(req)?;

    match result.choices[0].finish_reason.clone().unwrap() {
        chat_completion::FinishReason::stop => {
            println!("Stop");
            println!("{:?}", result.choices[0].message.clone().unwrap().content);
        }
        chat_completion::FinishReason::length => {
            println!("Length");
        }
        chat_completion::FinishReason::function_call => {
            println!("FunctionCall");
            #[derive(Serialize, Deserialize, Clone)]
            struct Currency {
                coin: String,
            }
            let message = result.choices[0].message.clone();
            let message = message.unwrap();
            let function_call = message.function_call.as_ref().unwrap();
            let name = function_call.name.clone().unwrap();
            let arguments = function_call.arguments.clone().unwrap();
            let c: Currency = serde_json::from_str(&arguments)?;
            let coin = c.coin;
            if name == "get_coin_price" {
                let price = get_coin_price(&coin);
                println!("{} price: {}", coin, price);
            }
        }
        chat_completion::FinishReason::content_filter => {
            println!("ContentFilter");
        }
        chat_completion::FinishReason::null => {
            println!("Null");
        }
    }
    Ok(())
}

// OPENAI_API_KEY=xxxx cargo run --package openai-api-rs --example function_call
