mod summarizer;
mod synthesizer;
use std::{collections::HashMap, path::PathBuf};

use clap::Parser;

use crate::{summarizer::FileSummarizer, synthesizer::Synthesizer};

#[derive(Parser, Debug)]
struct Args {
    pub paths: Vec<PathBuf>,
    #[arg(long)]
    pub depth: Option<u8>,
}

const MODEL_NAME: &str = "summarizer"; // Change to use config file
const MAX_DEPTH: u8 = 6;

fn get_files(path: PathBuf, depth: u8) -> Vec<PathBuf> {
    if depth == 0 {
        return vec![];
    }
    if path.is_file() {
        return vec![path];
    } else {
        let mut files = Vec::new();
        let path = &path;
        if let Ok(read_dir) = path.read_dir() {
            for entry in read_dir {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    files.extend(get_files(path, depth - 1));
                }
            }
        }
        files
    }
}

fn remove_think(src: String) -> String {
    let parts: Vec<&str> = src.split("<think>").collect();
    if parts.len() == 1 {
        return src;
    } else {
        parts[0].to_string()
            + &parts[1..]
                .iter()
                .map(|p| p.split("</think>").nth(1).unwrap_or(&p))
                .collect::<Vec<&str>>()
                .join("")
                .trim()
    }
}

#[tokio::main]
async fn main() {
    let Args { paths, depth } = Args::parse();
    // TODO : limit limit_size

    let mut files = Vec::new();
    for path in paths {
        files.extend(get_files(path, depth.unwrap_or(MAX_DEPTH)));
    }

    if files.len() == 1 {
        let summarizer = FileSummarizer::new();
        if let Ok(response) = summarizer
            .summarize_file(MODEL_NAME, files[0].clone())
            .await
        {
            println!("{}", remove_think(response.message.content));
        }
    } else if files.is_empty() {
        println!("No file to summarize");
    } else {
        let mut summaries = HashMap::new();
        let summarizer = FileSummarizer::new();
        let synthesizer = Synthesizer::new();
        for file in files {
            let summary = remove_think(
                summarizer
                    .summarize_file(MODEL_NAME, file.clone())
                    .await
                    .unwrap()
                    .message
                    .content,
            );
            summaries.insert(file, summary);
        }
        let synthesis = remove_think(
            synthesizer
                .synthesize(MODEL_NAME, summaries)
                .await
                .unwrap()
                .message
                .content,
        );
        println!("{}", synthesis);
    }
}
