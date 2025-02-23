mod constants;
mod utils;
mod biblio;

use utils::{format_apa, generate_sample};
use biblio::{BiblioError, extract_metadata};

use std::{env, fs, path::PathBuf};
use dotenv::dotenv;
use google_generative_ai_rs::v1::api::Client;
use google_generative_ai_rs::v1::gemini::Model;
use tokio::runtime::Runtime;

fn main() -> Result<(), BiblioError> {
    dotenv().ok();

    let model = env::var("MODEL").map_err(|_| BiblioError::IoError(std::io::Error::new(
        std::io::ErrorKind::NotFound, "`MODEL` key must be set in .env."
    )))?;

    let api_key = env::var("API_KEY").map_err(|_| BiblioError::IoError(std::io::Error::new(
        std::io::ErrorKind::NotFound, "`API_KEY` key must be set in .env."
    )))?;

    let client = Client::new_from_model(Model::Custom(model), api_key);

    let args: Vec<PathBuf> = env::args_os().skip(1).map(PathBuf::from).collect();
    if args.is_empty() {
        eprintln!("No files specified.");
        return Ok(());
    }

    let rt = Runtime::new().map_err(BiblioError::IoError)?;
    for path_raw in &args {
        let path = fs::canonicalize(path_raw).map_err(BiblioError::IoError)?;
        
        let sample = generate_sample(&path.to_str().unwrap(), &[1])?;
        let metadata = rt.block_on(extract_metadata(&client, sample))?;

        let new_name = format_apa(&metadata) + ".pdf";
        let new_path = path.with_file_name(&new_name);

        fs::rename(path, &new_path).map_err(BiblioError::IoError)?;
        println!("Renamed to {:?}", new_name);
    }

    Ok(())
}
