use google_generative_ai_rs::v1::{
    api::Client,
    gemini::{Content, Part, Role},
    gemini::request::{Request, GenerationConfig, SystemInstructionContent, SystemInstructionPart},
};
use crate::constants::{MAX_OUTPUT_TOKENS, MAX_TIMEOUT_SECONDS, PROMPT, TEMPERATURE, TOP_K, TOP_P};
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Represents bibliographic metadata extracted from a document
#[derive(Debug, Deserialize, Serialize)]
pub struct BiblioResponse {
    pub authors: Option<Vec<String>>,
    pub title: Option<String>,
    pub year: Option<String>,
}

/// Errors that can occur during bibliographic operations
#[derive(Debug, Error)]
pub enum BiblioError {
    #[error("Environment error: {0}")]
    ENVError(String),
    
    #[error("PDF processing error: {0}")]
    PDFError(String),
    
    #[error("Gemini API error: {0}")]
    GeminiError(String),
    
    #[error("JSON parsing error: {0}")]
    JSONError(#[from] serde_json::Error),
    
    #[error("I/O error: {0}")]
    IOError(#[from] std::io::Error),
}

/// Extracts bibliographic metadata from document text samples using the Gemini API
/// 
/// # Arguments
/// * `client` - The Gemini API client to use for requests
/// * `samples` - Vector of text samples extracted from documents
///
/// # Returns
/// A vector of bibliographic metadata responses corresponding to each input sample
pub async fn parse_document_metadata(
    client: &Client, 
    samples: Vec<String>
) -> Result<Vec<BiblioResponse>, BiblioError> {
    // Create a content object for each sample
    let contents = samples.into_iter()
        .map(|sample| Content {
            role: Role::User,
            parts: vec![Part {
                text: Some(sample),
                inline_data: None,
                file_data: None,
                video_metadata: None,
            }],
        })
        .collect();

    // Configure the API request
    let request = build_api_request(contents);

    // Send the request to the Gemini API
    let response = client.post(MAX_TIMEOUT_SECONDS, &request)
        .await
        .map_err(|e| BiblioError::GeminiError(format!("Could not connect to Gemini: {}", e)))?;

    // Extract the text from the response
    let response_text = response
        .rest()
        .and_then(|rest| {
            rest.candidates
                .get(0)
                .and_then(|c| c.content.parts.get(0))
                .and_then(|p| p.text.clone())
        })
        .unwrap_or_default();

    // Parse the JSON response
    serde_json::from_str(&response_text).map_err(BiblioError::JSONError)
}

/// Builds the API request with appropriate configuration
fn build_api_request(contents: Vec<Content>) -> Request {
    Request {
        contents,
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
    }
}