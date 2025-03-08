mod biblio;
mod constants;
mod utils;

use biblio::{parse_document_metadata, BiblioError};
use constants::BATCH_SIZE;
use google_generative_ai_rs::v1::{api::Client, gemini::Model};
use std::{
    env,
    fs,
    io::{self, BufRead}, 
    path::PathBuf,
    sync::Arc
};
use tokio::{
    sync::mpsc,
    task,
};
use utils::{format_filename, extract_pdf_sample, load_config};

type SampleResult = Result<(PathBuf, String), BiblioError>;

#[tokio::main]
async fn main() -> Result<(), BiblioError> {
    let env_path = env::current_exe()?
        .parent()
        .expect("Error: Failed to get executable directory")
        .join(".env");

    // Load the .env file if it exists
    if env_path.exists() {
        dotenv::from_path(&env_path).ok();
    } else {
        eprintln!("Error: .env file not found at {}", env_path.display());
        return Ok(());
    }
    
    let config = load_config()?;
    let client = Arc::new(Client::new_from_model(Model::Custom(config.model.clone()), config.api_key.clone()));
    
    let mut args: Vec<PathBuf> = env::args_os().skip(1).map(PathBuf::from).collect();
    
    // Read from stdin if no args are provided
    if args.is_empty() {
        let stdin = io::stdin();
        args = stdin
            .lock()
            .lines()
            .filter_map(Result::ok)
            .map(PathBuf::from)
            .collect();
    }
    
    if args.is_empty() {
        eprintln!("Error: No files specified.\nUsage: biblio <file1.pdf> <file2.pdf> ...");
        return Ok(());
    }

    println!("Processing {} files", args.len());
    
    let (tx, rx) = mpsc::channel::<SampleResult>(args.len());
    
    for path_raw in args {
        let tx = tx.clone();
        task::spawn(async move {
            let path = match fs::canonicalize(&path_raw) {
                Ok(p) => p,
                Err(e) => {
                    let _ = tx.send(Err(BiblioError::IOError(e))).await;
                    return;
                }
            };
            
            let path_str = path.to_str().unwrap_or_default().to_string();
            
            match extract_pdf_sample(&path_str, &[1, 2]) {
                Ok(sample) => {
                    let _ = tx.send(Ok((path, sample))).await;
                }
                Err(e) => {
                    let _ = tx.send(Err(e)).await;
                }
            }
        });
    }
    
    drop(tx);
    
    process_samples(rx, client, config.format).await?;
    
    Ok(())
}

async fn process_samples(
    mut rx: mpsc::Receiver<SampleResult>,
    client: Arc<Client>,
    format_template: String,
) -> Result<(), BiblioError> {
    let mut current_batch = Vec::new();
    
    while let Some(result) = rx.recv().await {
        match result {
            Ok(item) => {
                current_batch.push(item);
                
                if current_batch.len() >= BATCH_SIZE {
                    process_batch(&current_batch, &client, &format_template).await;
                    current_batch.clear();
                }
            }
            Err(e) => {
                // Log error but continue processing other files
                eprintln!("Error processing file: {}", e);
            }
        }
    }
    
    // Process any remaining items
    if !current_batch.is_empty() {
        process_batch(&current_batch, &client, &format_template).await;
    }
    
    Ok(())
}

async fn process_batch(
    batch: &[(PathBuf, String)], 
    client: &Client,
    format_template: &str,
) {
    let paths: Vec<_> = batch.iter().map(|(path, _)| path.clone()).collect();
    let samples: Vec<_> = batch.iter().map(|(_, sample)| sample.clone()).collect();
    
    match parse_document_metadata(client, samples).await {
        Ok(metadata) => {
            for (i, meta) in metadata.iter().enumerate() {
                if i >= paths.len() { continue; }
                
                let path = &paths[i];
                let filename = path.file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or_default();
                
                let new_filename = format_filename(meta, format_template) + ".pdf";
                let new_path = path.with_file_name(&new_filename);
                
                match fs::rename(path, &new_path) {
                    Ok(_) => println!(r#"Renamed: "{}" → "{}""#, filename, new_filename),
                    Err(e) => eprintln!("Failed to rename {}: {}", filename, e),
                }
            }
        }
        Err(e) => {
            // Log the error but continue with other batches
            eprintln!("Metadata extraction failed: {}", e);
            
            // Try to process each file individually to salvage what we can
            for (_i, (path, sample)) in batch.iter().enumerate() {
                let filename = path.file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or_default();
                
                println!("Attempting individual processing for: {}", filename);
                
                // Attempt individual extraction
                match parse_document_metadata(client, vec![sample.clone()]).await {
                    Ok(meta) if !meta.is_empty() => {
                        let new_filename = format_filename(&meta[0], format_template) + ".pdf";
                        let new_path = path.with_file_name(&new_filename);
                        
                        match fs::rename(path, &new_path) {
                            Ok(_) => println!("Renamed: {} → {}", filename, new_filename),
                            Err(e) => eprintln!("Failed to rename {}: {}", filename, e),
                        }
                    },
                    Ok(_) => eprintln!("No metadata received for {}", filename),
                    Err(e) => eprintln!("Individual processing failed for {}: {}", filename, e),
                }
            }
        }
    }
}