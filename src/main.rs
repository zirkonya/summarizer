mod summarizer;
mod synthesizer;
pub mod utils;

use clap::Parser;
use std::{collections::HashMap, path::PathBuf};
use zr_app::config_builder;

use crate::{
    summarizer::FileSummarizer,
    synthesizer::Synthesizer,
    utils::{get_files, remove_think},
};

#[derive(Parser, Debug)]
struct Args {
    pub paths: Vec<PathBuf>,
    #[arg(long)]
    pub depth: Option<u8>,
}

config_builder! {
    Conf {
        model: Model {
            name: String = "summarizer",
        },
        recusive: Recusive {
            default_depth: u8 = 8,
        }
    }
}

#[tokio::main]
#[zr_app::app(conf = Conf, app_folder = "~/.summarize")]
async fn main() {
    let Args { paths, depth } = Args::parse();
    let depth = depth.unwrap_or(config.recusive.default_depth);

    let mut files = Vec::new();
    for path in paths {
        files.extend(get_files(path, depth));
    }

    match files.len() {
        0 => println!("No file to summarize"),
        1 => {
            let summarizer = FileSummarizer::new();
            if let Ok(response) = summarizer
                .summarize_file(&config.model.name, files[0].clone())
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
                        .summarize_file(&config.model.name, file.clone())
                        .await
                        .unwrap()
                        .message
                        .content,
                );
                summaries.insert(file, summary);
            }
            let synthesis = remove_think(
                &synthesizer
                    .synthesize(&config.model.name, summaries)
                    .await
                    .unwrap()
                    .message
                    .content,
            );
            println!("{synthesis}");
        }
    }
}
