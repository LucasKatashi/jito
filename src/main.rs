use clap::Parser;
use serde::Deserialize;
use serde_yaml::Error;
use std::fs::File;
use std::io::{self, Read};
use text_colorizer::*;

#[derive(Debug, Parser)]
struct Arguments {
    #[arg(long, short)]
    output: String,

    #[arg(long, short, default_value = "webhooks.yaml")]
    file: String,
}

#[derive(Debug, Deserialize)]
struct Webhooks {
    discord: String,
    slack: String,
}

fn webhook_parser(file: &str) -> Result<Webhooks, Error> {
    let mut file = File::open(&file).expect("Unable to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read file");
    serde_yaml::from_str(&contents)
}

async fn sender(output: &str, content: &str, file: &str) {
    let webhooks = webhook_parser(&file).expect("Unable to parse webhooks");
    let client = reqwest::Client::new();

    let chunks = content
        .chars()
        .collect::<Vec<char>>()
        .chunks(2000)
        .map(|c| c.iter().collect::<String>())
        .collect::<Vec<String>>();

    let mut success = true;

    for chunk in chunks {
        match output {
            "discord" => {
                let message = serde_json::json!({
                    "content": chunk
                });

                let res = client
                    .post(&webhooks.discord)
                    .header("Content-Type", "application/json")
                    .json(&message)
                    .send()
                    .await;
                if let Err(e) = res {
                    success = false;
                    eprintln!(
                        "{} Failed to send message to Discord: {}",
                        "[-]".red().bold(),
                        e
                    );
                }
            }
            "slack" => {
                let message = serde_json::json!({
                    "text": chunk
                });

                let res = client
                    .post(&webhooks.slack)
                    .header("Content-Type", "application/json")
                    .json(&message)
                    .send()
                    .await;
                if let Err(e) = res {
                    success = false;
                    eprintln!(
                        "{} Failed to send message to Slack: {}",
                        "[-]".red().bold(),
                        e
                    );
                }
            }
            _ => eprintln!("{} Invalid output", "[-]".red().bold()),
        }
    }

    if success {
        println!("{} Message sent to {}", "[+]".green().bold(), output);
    }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let args = Arguments::parse();
    let mut buffer = String::new();

    let _ = io::stdin().read_to_string(&mut buffer)?;

    sender(&args.output, &buffer, &args.file).await;

    Ok(())
}
