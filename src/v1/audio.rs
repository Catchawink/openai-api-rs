use serde::{Deserialize, Serialize};

use crate::impl_builder_methods;

pub const WHISPER_1: &str = "whisper-1";

#[derive(Debug, Serialize, Clone)]
pub struct AudioSpeechRequest {
    pub model: String,
    pub input: String,
    pub voice: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed: Option<f32>,
}

impl AudioSpeechRequest {
    pub fn new(model: String, input: String, voice: String) -> Self {
        Self {
            model,
            input,
            voice,
            response_format: None,
            speed: None,
        }
    }
}

impl_builder_methods!(
    AudioSpeechRequest,
    response_format: String,
    speed: f32
);

#[derive(Debug, Serialize, Clone)]
pub struct AudioTranscriptionRequest {
    pub file: String,
    pub model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
}

impl AudioTranscriptionRequest {
    pub fn new(file: String, model: String) -> Self {
        Self {
            file,
            model,
            prompt: None,
            response_format: None,
            temperature: None,
            language: None,
        }
    }
}

impl_builder_methods!(
    AudioTranscriptionRequest,
    prompt: String,
    response_format: String,
    temperature: f32,
    language: String
);

#[derive(Debug, Deserialize, Clone)]
pub struct AudioTranscriptionResponse {
    pub text: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct AudioTranslationRequest {
    pub file: String,
    pub model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
}

impl AudioTranslationRequest {
    pub fn new(file: String, model: String) -> Self {
        Self {
            file,
            model,
            prompt: None,
            response_format: None,
            temperature: None,
        }
    }
}

impl_builder_methods!(
    AudioTranslationRequest,
    prompt: String,
    response_format: String,
    temperature: f32
);

#[derive(Debug, Deserialize, Clone)]
pub struct AudioTranslationResponse {
    pub text: String,
}
