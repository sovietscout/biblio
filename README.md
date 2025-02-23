# Biblio
Biblio is a command-line tool for extracting metadata from academic PDFs and renaming them in an APA-compliant format. It automates the process of organizing research papers by leveraging Google's Gemini family of LLMs to extract author names, titles, and publication years.

## Features
- Extracts metadata (authors, title, and year) from PDFs.
- Uses Google Gemini LLM for intelligent text extraction.
- Renames PDFs in APA-compliant format.
- Handles batch processing efficiently.
- Skips files with missing metadata.
- Prevents filename conflicts by sanitizing invalid characters.

## Installation
### Option 1: Precompiled Executable (Recommended for Windows)
Download the latest `.exe` release from GitHub Releases and place it in a directory included in your system's `PATH`.

### Option 2: Build from Source

#### Prerequisites
- Rust (latest stable version)
- Google Gemini API Key
- `dotenv` for environment variable management
- `lopdf` for PDF text extraction
- `serde` for JSON parsing

#### Steps
1. Clone the repository:
   ```sh
   git clone https://github.com/sovietscout/biblio.git
   cd biblio
   ```
2. Set up environment variables in a `.env` file:
   ```sh
   API_KEY=your_google_gemini_api_key
   MODEL=your_gemini_model # gemini-2.0-flash-lite-preview-02-05
   ```
3. Build and run the project:
   ```sh
   cargo build --release
   ./target/release/biblio <pdf-files>
   ```

## Usage
```sh
biblio file1.pdf file2.pdf ...
```
- Processes multiple PDFs at once.
- Extracted metadata is used to rename the files automatically.
- Example output:
  ```sh
  Renamed "paper1.pdf" to "Smith, J. (2020). Research Study.pdf"
  Renamed "paper2.pdf" to "Doe, J., & Brown, A. (2018). AI in Healthcare.pdf"
  ```

## Error Handling
- If a PDF has no extractable text, it is skipped.
- If the LLM fails to generate metadata, an error is logged.
- Invalid characters in filenames are replaced with underscores.

## License
MIT License. See `LICENSE` file for details.
