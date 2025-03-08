# Biblio
Biblio is a simple command-line tool for managing academic PDFs. It extracts metadata (title, authors, year) and renames files automatically based on a customisable format.

## 🚀 Features
- **Automated Metadata Extraction**: Extracts titles, authors, and years from PDFs using the Google Gemini API.
- **Batch Processing**: Handles multiple files at once for faster results.
- **Customisable File Naming**: Renames files in a structured format you define.
- **Hassle-Free Organisation**: Prevents duplicate filenames and keeps your files tidy.

## 🔧 Installation

### Windows (Standalone Binary)
1. Download the latest `.exe` file from the [Releases](https://github.com/sovietscout/biblio/releases/) page.
2. Place it in a preferred directory for easy access. [Optionally, add the folder to `System Properties > Advanced > Environment Variables > PATH` for global access]
3. Run the .exe directly from the command line or by double-clicking it.

### Linux/macOS (Build from Source)
1. **Clone the repository:**
   ```sh
   git clone https://github.com/sovietscout/biblio.git
   cd biblio
   ```
2. **Build the project:**
   ```sh
   cargo build --release
   ```

## 📄 Configuration
Create a `.env` file in the project root:
```properties
# API Configuration
MODEL=gemini-2.0-flash-lite
API_KEY=YOUR_GEMINI_API_KEY

# Output Formatting
FORMAT="{authors} ({year}). {title}"
```
**Customising the Format:**
- You can define how renamed files should be structured using placeholders:
 - `{authors}` → Author names (defaults to `Unknown Author` if not found).
 - `{year}` → Publication year (defaults to `Unknown Year` if missing).
 - `{title}` → Document title (defaults to `Untitled` if missing).
- Example format: `FORMAT="{authors} - {title} ({year})"` renames a file to `Smith, J. - Research Study (2020).pdf`.

## 🔑 Getting a Google Gemini API Key
1. Go to [Google AI Studio](https://aistudio.google.com/).
2. Sign in with your Google account.
3. Navigate to the 'Get API key' section and create a new API key.
4. Copy the API key and add it to your `.env` file as `API_KEY`.

## 📂 Usage
### Basic Usage
```sh
biblio file1.pdf file2.pdf ...
```
### Piping for Bulk Processing
Instead of listing files manually, piping lets you process all PDFs in a folder:

**Windows (PowerShell)**
```powershell
Get-ChildItem -Path . -Filter *.pdf | ForEach-Object { $_.FullName } | biblio
```

**Mac/Linux**
```bash
ls *.pdf | biblio
```

## 📜License
MIT License. See `LICENSE` for details.

