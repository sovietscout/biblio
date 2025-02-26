# Biblio
Biblio is a command-line tool for extracting metadata from academic PDFs and renaming them in accordinig to a customisable format. It leverages the Google Gemini API for metadata extraction and provides a flexible way to manage your academic or research PDF library.

## Features
- Extracts metadata (authors, title, and year) from PDFs.
- Uses Google Gemini LLM for intelligent text extraction.
- Renames PDFs in APA-compliant format.
- Handles batch processing efficiently.
- Skips files with missing metadata.
- Prevents filename conflicts by sanitizing invalid characters.

## Installation

1.  **Clone the Repository:**
    ```bash
    git clone <repository_url>
    cd biblio
    ```

2.  **Create the `.env` File:**
    Create a `.env` file in the root directory of the project. Add the following content, replacing the placeholders with your actual values:
    ```properties
    MODEL = gemini-2.0-flash-lite
    API_KEY = YOUR_GEMINI_API_KEY
    FORMAT = "{authors} ({year}). {title}"
    ```
    - **MODEL:** The name of the Gemini model to use (e.g., `gemini-2.0-flash-lite`).
    - **API_KEY:** Your Google Gemini API key.
    - **FORMAT_STR:** The format string for renaming files. You can use the following placeholders:
        -   `{authors}`: The authors of the document.
        -   `{title}`: The title of the document.
        -   `{year}`: The year of the document.
        - if these properties are not found, default values are used. For example: `{authors}` defaults to `Unknown Author`.

3.  **Build the Project:**
    ```bash
    cargo build --release
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

## License
MIT License. See `LICENSE` file for details.
