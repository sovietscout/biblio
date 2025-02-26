use lopdf::Document;
use crate::biblio::{BiblioResponse, BiblioError};

pub fn format_apa(resp: &BiblioResponse) -> String {
    let raw_authors = resp.authors.clone().unwrap_or_default();

    let authors = match raw_authors.len() {
        0 => "Unknown".to_string(),
        1 => raw_authors[0].clone(),
        2 => format!("{}, & {}", raw_authors[0], raw_authors[1]),
        _ => format!("{}, et al.", raw_authors[0]),
    };

    let year = resp.year.as_deref().unwrap_or("n.d.");

    let title = resp.title.as_deref()
        .map(|t| t.chars().map(|c| if r#"<>:"/\|?*"#.contains(c) { '_' } else { c }).collect::<String>())
        .unwrap_or_else(|| "Untitled".to_string());

    format!("{} ({}). {}", authors, year, title)
}

pub fn generate_sample(path: &str, pages: &[u32]) -> Result<String, BiblioError> {
    let doc = Document::load(path)
        .map_err(|e| BiblioError::PDFError(format!("Failed to load PDF: {}", e)))?;

    let text = doc.extract_text(pages)
        .map_err(|e| BiblioError::PDFError(format!("Failed to extract text from PDF: {}", e)))?;

    Ok(text)
}
