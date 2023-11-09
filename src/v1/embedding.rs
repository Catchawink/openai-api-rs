use serde::{Deserialize, Serialize};
use std::option::Option;

use crate::impl_builder_methods;

#[derive(Debug, Deserialize, Clone)]
pub struct EmbeddingData {
    pub object: String,
    pub embedding: Vec<f32>,
    pub index: i32,
}

#[derive(Debug, Serialize)]
pub struct EmbeddingRequest {
    pub model: String,
    pub input: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

impl EmbeddingRequest {
    pub fn new(model: String, input: String) -> Self {
        Self {
            model,
            input,
            user: None,
        }
    }
}

impl_builder_methods!(
    EmbeddingRequest,
    user: String
);

#[derive(Debug, Deserialize, Clone)]
pub struct EmbeddingResponse {
    pub object: String,
    pub data: Vec<EmbeddingData>,
    pub model: String,
    pub usage: Usage,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Usage {
    pub prompt_tokens: i32,
    pub total_tokens: i32,
}
