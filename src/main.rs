mod summarizer;
mod synthesizer;
mod utils;

use crate::{
    summarizer::FileSummarizer,
    synthesizer::Synthesizer,
    utils::{get_files, remove_think},
};
use clap::Parser;
use std::{collections::HashMap, path::PathBuf};

#[derive(Parser, Debug)]
struct Args {
    pub paths: Vec<PathBuf>,
    #[arg(long)]
    pub depth: Option<u8>,
}

const MODEL_NAME: &str = "summarizer"; // Change to use config file
const MAX_DEPTH: u8 = 6;

#[tokio::main]
async fn main() {
    let Args { paths, depth } = Args::parse();

    let mut files = Vec::new();
    for path in paths {
        files.extend(get_files(path, depth.unwrap_or(MAX_DEPTH)));
    }

    match files.len() {
        0 => println!("No file to summarize"),
        1 => {
            let summarizer = FileSummarizer::new();
            if let Ok(response) = summarizer
                .summarize_file(MODEL_NAME, files[0].clone())
                .await
            {
                println!("{}", remove_think(&response.message.content));
            }
        }
        _ => {
            let mut summaries = HashMap::new();
            let summarizer = FileSummarizer::new();
            let synthesizer = Synthesizer::new();
            for file in files {
                let summary = remove_think(
                    &summarizer
                        .summarize_file(MODEL_NAME, file.clone())
                        .await
                        .unwrap()
                        .message
                        .content,
                );
                summaries.insert(file, summary);
            }
            let synthesis = remove_think(
                &synthesizer
                    .synthesize(MODEL_NAME, summaries)
                    .await
                    .unwrap()
                    .message
                    .content,
            );
            println!("{}", synthesis);
        }
    }
}
