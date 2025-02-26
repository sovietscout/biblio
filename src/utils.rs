use std::{collections::HashMap, env};

use lopdf::Document;
use crate::biblio::{BiblioResponse, BiblioError};

pub(crate) struct Config {
    pub format: String,
    pub model: String,
    pub api_key: String,
}

pub fn load_config() -> Result<Config, BiblioError> {
    let model_name = env::var("MODEL").map_err(|_| {
        eprintln!("Error: `MODEL` key not found in .env. Please set it to your Gemini model.");
        BiblioError::ENVError("`MODEL` key not found in .env".to_string())
    })?;

    let api_key = env::var("API_KEY").map_err(|_| {
        eprintln!("Error: `API_KEY` key not found in .env. Please set your Google Gemini API key.");
        BiblioError::ENVError("`API_KEY` key not found in .env".to_string())
    })?;

    let format_string = match env::var("FORMAT") {
        Ok(format) => format,
        Err(_) => {
            eprintln!("Warning: `FORMAT` key not found in .env. Using default format string.");
            "{authors} ({year}). {title}".to_string()
        }
    };
    
    Ok(Config { format: format_string, model: model_name, api_key: api_key })
}

pub fn format_custom(data: &BiblioResponse, format_str: &str) -> String {
    let sanitize = |s: &str| {
        s.chars()
            .map(|c| match c {
                '<' | '>' | ':' | '"' | '/' | '\\' | '|' | '?' | '*' => '_',
                _ => c,
            })
            .collect::<String>()
    };

    let authors = match data.authors.as_deref() {
        Some([]) | None => "Unknown Author".to_string(),
        Some([one]) => one.clone(),
        Some([first, second]) => format!("{} & {}", first, second),
        Some([first, ..]) => format!("{} et al.", first),
    };

    let title = data.title.as_deref().unwrap_or("Untitled").to_string();
    let year = data.year.as_deref().unwrap_or("Unknown Year").to_string();

    let replacements = HashMap::from([
        ("authors", sanitize(&authors)),
        ("title", sanitize(&title)),
        ("year", sanitize(&year)),
    ]);

    let mut result = String::new();
    let mut chars = format_str.chars().peekable();

    while let Some(&c) = chars.peek() {
        if c == '{' {
            chars.next(); // Consume '{'
            
            if chars.peek() == Some(&'{') {
                // Found escaped '{{', keep a single '{'
                result.push('{');
                chars.next(); // Consume second '{'
                continue;
            }

            let mut placeholder = String::new();
            while let Some(&next_c) = chars.peek() {
                if next_c == '}' {
                    chars.next(); // Consume '}'
                    break;
                }
                placeholder.push(next_c);
                chars.next();
            }

            if let Some(replacement) = replacements.get(placeholder.as_str()) {
                result.push_str(replacement);
            } else {
                result.push_str(&format!("{{{}}}", placeholder)); // Preserve unknown placeholders
            }
        } else if c == '}' {
            chars.next(); // Consume '}'

            if chars.peek() == Some(&'}') {
                // Found escaped '}}', keep a single '}'
                result.push('}');
                chars.next(); // Consume second '}'
            } else {
                // Unmatched '}', treat as literal
                result.push('}');
            }
        } else {
            result.push(c);
            chars.next();
        }
    }

    result
}

pub fn generate_sample(path: &str, pages: &[u32]) -> Result<String, BiblioError> {
    let doc = Document::load(path)
        .map_err(|e| BiblioError::PDFError(format!("Failed to load PDF: {}", e)))?;

    let text = doc.extract_text(pages)
        .map_err(|e| BiblioError::PDFError(format!("Failed to extract text from PDF: {}", e)))?;

    Ok(text)
}
