use std::{collections::HashMap, env, path::Path};
use lopdf::Document;
use crate::biblio::{BiblioResponse, BiblioError};

/// Configuration for the application
#[derive(Clone, Debug)]
pub struct Config {
    pub format: String,
    pub model: String,
    pub api_key: String,
}

/// Loads configuration from environment variables
pub fn load_config() -> Result<Config, BiblioError> {
    let model_name = env::var("MODEL").map_err(|_| {
        BiblioError::ENVError("MODEL key not found in .env file".to_string())
    })?;
    
    let api_key = env::var("API_KEY").map_err(|_| {
        BiblioError::ENVError("API_KEY not found in .env file".to_string())
    })?;
    
    let format_string = env::var("FORMAT").unwrap_or_else(|_| {
        // Default format if not specified
        "{authors} ({year}). {title}".to_string()
    });
   
    Ok(Config { 
        format: format_string, 
        model: model_name, 
        api_key 
    })
}

/// Formats bibliographic data according to a template string
pub fn format_filename(data: &BiblioResponse, format_str: &str) -> String {
    let authors = match data.authors.as_deref() {
        Some([]) | None => "Unknown Author".to_string(),
        Some([one]) => one.clone(),
        Some([first, second]) => format!("{} & {}", first, second),
        Some([first, ..]) => format!("{} et al.", first),
    };
    
    let title = data.title.as_deref().unwrap_or("Untitled").to_string();
    
    let year = data.year.as_deref().unwrap_or("Unknown Year").to_string();
    
    let replacements = HashMap::from([
        ("authors", sanitize_filename(&authors)),
        ("title", sanitize_filename(&title)),
        ("year", sanitize_filename(&year)),
    ]);

    
    // Process the format string with replacements
    process_format_string(format_str, &replacements)
}

/// Sanitizes a string for use in filenames
fn sanitize_filename(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            '<' | '>' | ':' | '"' | '/' | '\\' | '|' | '?' | '*' => '_',
            _ => c,
        })
        .collect()
}

/// Processes a format string, replacing placeholders with values
fn process_format_string(format_str: &str, replacements: &HashMap<&'static str, String>) -> String {
    let mut result = String::with_capacity(format_str.len() * 2);
    let mut chars = format_str.chars().peekable();
    
    while let Some(&c) = chars.peek() {
        match c {
            '{' => {
                chars.next(); // Consume '{'
                
                // Handle escaped braces
                if chars.peek() == Some(&'{') {
                    result.push('{');
                    chars.next(); // Consume second '{'
                    continue;
                }
                
                // Collect placeholder name
                let mut placeholder = String::new();
                while let Some(&next_c) = chars.peek() {
                    if next_c == '}' {
                        chars.next(); // Consume '}'
                        break;
                    }
                    placeholder.push(next_c);
                    chars.next();
                }
                
                // Replace with value or keep placeholder if not found
                if let Some(replacement) = replacements.get(placeholder.as_str()) {
                    result.push_str(replacement);
                } else {
                    result.push('{');
                    result.push_str(&placeholder);
                    result.push('}');
                }
            },
            '}' => {
                chars.next(); // Consume '}'
                
                // Handle escaped braces
                if chars.peek() == Some(&'}') {
                    result.push('}');
                    chars.next(); // Consume second '}'
                } else {
                    // Unmatched '}', treat as literal
                    result.push('}');
                }
            },
            _ => {
                result.push(c);
                chars.next();
            }
        }
    }
    
    result
}

/// Extracts text from specified pages of a PDF file
pub fn extract_pdf_sample<P: AsRef<Path>>(path: P, pages: &[u32]) -> Result<String, BiblioError> {
    let doc = Document::load(path.as_ref())
        .map_err(|e| BiblioError::PDFError(format!("Failed to load PDF: {}", e)))?;
    
    let text = doc.extract_text(pages)
        .map_err(|e| BiblioError::PDFError(format!("Failed to extract text: {}", e)))?;
    
    Ok(text)
}