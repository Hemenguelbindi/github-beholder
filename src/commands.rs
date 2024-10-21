extern crate reqwest;
use clap::Parser;
use serde::{Deserialize, Serialize};
use reqwest::header;
use std::error::Error;
use serde_json;
use crate::config::Config;

#[derive(Serialize, Deserialize, Debug)]
struct Repo {
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Commit {
    message: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Payload {
    commits: Option<Vec<Commit>>,
    action: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Event {
    #[serde(rename = "type")]
    type_: String,
    repo: Repo,
    payload: Payload,
}

impl Event {
    fn display_activity(&self) {
        match self.type_.as_str() {
            "PushEvent" => {
                let commit_count = self.payload.commits.as_ref().map_or(0, |c| c.len());
                println!("Pushed {} commits to {}", commit_count, self.repo.name);
            }
            "IssuesEvent" => {
                if let Some(action) = &self.payload.action {
                    println!("{} a new issue in {}", action, self.repo.name);
                }
            }
            "WatchEvent" => {
                if let Some(action) = &self.payload.action {
                    if action == "started" {
                        println!("Starred {}", self.repo.name);
                    }
                }
            }
            _ => {
                println!("Unknown event type: {}", self.type_);
            }
        }
    }
}

#[derive(Parser)]
#[command(author = "Hemenguelbindi", version = "0.0.1", about = "CLI beholder for activity in GitHub", long_about = None)]
pub struct Cli {
    pub name: String,
}

impl Cli {
    pub async fn progress(&self) -> Result<(), Box<dyn Error>> {
        let config = Config::init();
        print_ascii_owl();
        println!("Проверяем активность пользователя: {}", self.name);

        let url = format!("https://api.github.com/users/{}/events", self.name);
        let mut headers = header::HeaderMap::new();
        headers.insert("Accept", "application/vnd.github+json".parse()?);
        headers.insert("User-Agent", "github-beholder".parse()?);

        let token = format!("Bearer {}", config.token);
        headers.insert("Authorization", token.parse()?);

        let client = reqwest::Client::new();
        let response = client.get(&url).headers(headers).send().await?;

        if response.status().is_success() {
            let response_text = response.text().await?;
            
            match serde_json::from_str::<Vec<Event>>(&response_text) {
                Ok(events) => {
                    if events.is_empty() {
                        println!("Нет активности для пользователя.");
                    } else {
                        for event in events {
                            event.display_activity();
                        }
                    }
                }
                Err(e) => {
                    println!("Ошибка при десериализации ответа API: {}", e);
                }
            }
        } else {
            println!("Ошибка: {} - {}", response.status(), response.text().await?);
        }

        Ok(())
    }
}

fn print_ascii_owl() {
    let art = r#"
      ___
     (o,o)
     { " }
     -"-"-
    "#;
    println!("Смотрим активность пользователя {}", art);
}
