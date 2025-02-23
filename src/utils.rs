use lopdf::Document;
use crate::biblio::{BiblioResponse, BiblioError};

pub fn format_apa(resp: &BiblioResponse) -> String {
    let authors = match resp.authors.len() {
        1 => resp.authors[0].clone(),
        2 => format!("{}, & {}", resp.authors[0], resp.authors[1]),
        _ => format!("{}, et al.", resp.authors[0]),
    };

    let year = if resp.year.is_empty() { "n.d." } else { &resp.year };

    let title = resp.title.chars()
        .map(|c| if r#"<>:"/\|?*"#.contains(c) { '_' } else { c })
        .collect::<String>();

    format!("{} ({}). {}", authors, year, title)
}

pub fn generate_sample(path: &str, pages: &[u32]) -> Result<String, BiblioError> {
    let doc = Document::load(path)
        .map_err(|e| BiblioError::PdfError(format!("Failed to load PDF: {}", e)))?;

    let text = doc.extract_text(pages)
        .map_err(|e| BiblioError::PdfError(format!("Failed to extract text from PDF: {}", e)))?;

    Ok(text)
}
