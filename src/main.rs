mod constants;
mod utils;
mod biblio;

use constants::BATCH_SIZE;
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
        std::io::ErrorKind::NotFound, "`MODEL` key must be set in .env"
    )))?;

    let api_key = env::var("API_KEY").map_err(|_| BiblioError::IoError(std::io::Error::new(
        std::io::ErrorKind::NotFound, "`API_KEY` key must be set in .env"
    )))?;

    let client = Client::new_from_model(Model::Custom(model), api_key);

    let args: Vec<PathBuf> = env::args_os().skip(1).map(PathBuf::from).collect();
    if args.is_empty() {
        eprintln!("No files specified");
        return Ok(());
    }
    
    if args.len() > BATCH_SIZE {
        println!("Batching in groups of {}", BATCH_SIZE);
    }

    let rt = Runtime::new().map_err(BiblioError::IoError)?;

    // Batch to avoid RPM limits
    for chunk in args.chunks(BATCH_SIZE) {
        let mut samples = Vec::new();
        let mut paths = Vec::new();

        for path_raw in chunk {
            let path = fs::canonicalize(path_raw).map_err(BiblioError::IoError)?;

            paths.push(path.clone());
            match generate_sample(&path.to_str().unwrap(), &[1, 2]) {
                Ok(text) => samples.push(text),
                Err(err) => eprintln!("Error extracting text from {:?}: {:?}", path_raw, err),
            }
        }

        if samples.is_empty() {
            continue;
        }

        match rt.block_on(extract_metadata(&client, samples)) {
            Ok(metadata) => {
                if paths.len() != metadata.len() {
                    eprintln!("Warning: Received {} metadata entires for {} files", metadata.len(), paths.len());
                }

                for (i, m) in metadata.iter().enumerate() {
                    if i >= paths.len() {
                        break;
                    }

                    let name = paths[i].file_name().unwrap_or_default().to_str().unwrap_or_default();
                    let name_fmt = format_apa(&m) + ".pdf";
                    let name_path = paths[i].with_file_name(name_fmt.clone());
                    
                    match fs::rename(&paths[i], &name_path) {
                        Ok(_) => println!(r#"Renamed "{:?}" to "{:?}""#, name, &name_fmt),
                        Err(err) => {
                            eprintln!("Failed to rename {:?}: {}", paths[i], err);
                        }
                    }
    
                }
            }
            Err(err) => {
                eprintln!("Failed to extract metadata for batch: {}", err);
            }
        }
    }

    Ok(())
}
