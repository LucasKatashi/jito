use clap::Parser;
use reqwest;
use serde::Deserialize;
use serde_yaml::Error;
use std::fs::File;
use std::io::{self, Read};

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

    match output {
        "discord" => {
            let message = serde_json::json!({
                "content": content
            });

            let res = client
                .post(&webhooks.discord)
                .header("Content-Type", "application/json")
                .json(&message)
                .send()
                .await;
            match res {
                Ok(_) => println!("Message sent to Discord"),
                Err(e) => eprintln!("Failed to send message to Discord: {}", e),
            }
        }
        "slack" => {
            let message = serde_json::json!({
                "text": content
            });

            let res = client
                .post(&webhooks.slack)
                .header("Content-Type", "application/json")
                .json(&message)
                .send()
                .await;
            match res {
                Ok(_) => println!("Message sent to Slack"),
                Err(e) => eprintln!("Failed to send message to Slack: {}", e),
            }
        }
        _ => eprintln!("Invalid output"),
    }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let args = Arguments::parse();
    let mut buffer = String::new();

    let _ = io::stdin().read_line(&mut buffer)?;

    sender(&args.output, &buffer, &args.file).await;

    Ok(())
}
