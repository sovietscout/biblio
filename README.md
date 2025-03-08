# Biblio
Biblio is a simple command-line tool for managing academic PDFs. It extracts metadata (title, authors, year) and renames files automatically based on a customizable format.

## 🚀 Features
- **Automated Metadata Extraction**: Extracts titles, authors, and years from PDFs using the Google Gemini API.
- **Batch Processing**: Handles multiple files at once for faster results.
- **Customisable File Naming**: Renames files in a structured format you define.
- **Hassle-Free Organisation**: Prevents duplicate filenames and keeps your files tidy.

## 🔧 Installation

### Option 1: Download the Windows Executable
1. Download the latest `.exe` file from the [Releases](https://github.com/sovietscout/biblio/releases/) page.
2. Place it in a preferred directory for easy access.
3. Run the .exe directly from the command line or by double-clicking it.

### Option 2: Build from Source (For Developers)
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
**Customizing the Format:**
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
```sh
biblio file1.pdf file2.pdf ...
```
- Processes multiple PDFs at once.
- Renames files using extracted metadata.

### Example
```sh
> biblio paper1.pdf paper2.pdf
Processing 2 files
Renamed: paper1.pdf → Smith, J. (2020). Research Study.pdf
Renamed: paper2.pdf → Doe, J., & Brown, A. (2018). AI in Healthcare.pdf
```

## 📜License
MIT License. See `LICENSE` for details.

