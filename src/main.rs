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

#[tokio::main]
async fn main() -> Result<(), BiblioError> {
    dotenv().ok();

    let model = env::var("MODEL").map_err(|_| {
        BiblioError::IoError(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "`MODEL` key must be set in .env",
        ))
    })?;
    
    let api_key = env::var("API_KEY").map_err(|_| {
        BiblioError::IoError(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "`API_KEY` key must be set in .env",
        ))
    })?;

    let client = Client::new_from_model(Model::Custom(model), api_key);

    let args: Vec<PathBuf> = env::args_os().skip(1).map(PathBuf::from).collect();
    if args.is_empty() {
        eprintln!("No files specified");
        return Ok(());
    }

    if args.len() >= BATCH_SIZE {
        println!("Batching in groups of {}", BATCH_SIZE);
    }

    let mut sample_tasks = Vec::new();
    for path_raw in args {
        let path = fs::canonicalize(&path_raw).map_err(BiblioError::IoError)?;
        let path_str = path.to_str().unwrap().to_string();
        let path_clone = path.clone();
        
        let task = tokio::task::spawn_blocking(move || {
            generate_sample(&path_str, &[1, 2]).map(|sample| (path_clone, sample))
        });
        sample_tasks.push(task);
    }

    let mut results = Vec::new();
    for task in sample_tasks {
        match task.await {
            Ok(Ok(pair)) => results.push(pair),
            Ok(Err(err)) => eprintln!("Error generating sample: {:?}", err),
            Err(err) => eprintln!("Task panicked: {:?}", err),
        }
    }

    for chunk in results.chunks(BATCH_SIZE) {
        let mut samples_batch = Vec::new();
        let mut paths_batch = Vec::new();
        for (path, sample) in chunk {
            paths_batch.push(path.clone());
            samples_batch.push(sample.clone());
        }

        if samples_batch.is_empty() {
            continue;
        }

        match extract_metadata(&client, samples_batch).await {
            Ok(metadata) => {
                
                if paths_batch.len() != metadata.len() {
                    eprintln!(
                        "Warning: Received {} metadata entries for {} files",
                        metadata.len(),
                        paths_batch.len()
                    );
                }

                for (i, m) in metadata.iter().enumerate() {
                    if i >= paths_batch.len() {
                        break;
                    }
                    
                    let name = paths_batch[i]
                        .file_name()
                        .unwrap_or_default()
                        .to_str()
                        .unwrap_or_default();
                    
                    let name_fmt = format_apa(&m) + ".pdf";
                    let name_path = paths_batch[i].with_file_name(name_fmt.clone());

                    match fs::rename(&paths_batch[i], &name_path) {
                        Ok(_) => println!("Renamed {:?} to {:?}", name, name_fmt),
                        Err(err) => eprintln!("Failed to rename {:?}: {}", paths_batch[i], err),
                    }
                }
            }
            Err(err) => eprintln!("Failed to extract metadata for batch: {}", err),
        }
    }

    Ok(())
}
