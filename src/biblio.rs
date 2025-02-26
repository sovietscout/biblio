use google_generative_ai_rs::v1::{
    api::Client,
    gemini::{Content, Part, Role},
    gemini::request::{Request, GenerationConfig, SystemInstructionContent, SystemInstructionPart},
};
use crate::constants::{MAX_OUTPUT_TOKENS, MAX_TIMEOUT_SECONDS, PROMPT, TEMPERATURE, TOP_K, TOP_P};
use serde::{Deserialize, Serialize};
use serde_json::from_str;

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct BiblioResponse {
    pub authors: Option<Vec<String>>,
    pub title: Option<String>,
    pub year: Option<String>,
}

#[derive(Debug)]
pub(crate) enum BiblioError {
    ENVError(String),
    PDFError(String),
    GeminiError(String),
    JSONError(serde_json::Error),
    IOError(std::io::Error),
}

impl std::fmt::Display for BiblioError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            BiblioError::ENVError(msg) => write!(f, "ENV Error: {}", msg),
            BiblioError::PDFError(msg) => write!(f, "PDF Error: {}", msg),
            BiblioError::GeminiError(msg) => write!(f, "Gemini API Error: {}", msg),
            BiblioError::JSONError(err) => write!(f, "JSON Parsing Error: {}", err),
            BiblioError::IOError(err) => write!(f, "IO Error: {}", err),
        }
    }
}

impl std::error::Error for BiblioError {}


pub async fn extract_metadata(client: &Client, samples: Vec<String>) -> Result<Vec<BiblioResponse>, BiblioError> {
    let contents = samples.into_iter().map(|sample| Content {
        role: Role::User,
        parts: vec![Part {
            text: Some(sample),
            inline_data: None,
            file_data: None,
            video_metadata: None,
        }],
    }).collect();

    let request = Request {
        contents: contents,
        tools: vec![],
        safety_settings: vec![],
        generation_config: Some(GenerationConfig {
            temperature: Some(TEMPERATURE),
            top_p: Some(TOP_P),
            top_k: Some(TOP_K),
            candidate_count: None,
            max_output_tokens: Some(MAX_OUTPUT_TOKENS),
            stop_sequences: None,
            response_mime_type: Some("application/json".to_string()),
            response_schema: None,
        }),
        system_instruction: Some(SystemInstructionContent {
            parts: vec![SystemInstructionPart {
                text: Some(PROMPT.to_string())
            }]
        }),
    };

    let response = client.post(MAX_TIMEOUT_SECONDS, &request).await
        .map_err(|_| BiblioError::GeminiError("Could not connect to Gemini.".to_string()))?;

    let response_text = response.rest()
        .unwrap()
        .candidates
        .get(0)
        .and_then(|c| c.content.parts.get(0))
        .and_then(|p| p.text.clone())
        .unwrap_or_default();

    from_str(&response_text).map_err(BiblioError::JSONError)
}
