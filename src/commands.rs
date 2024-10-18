extern crate reqwest;
use clap::Parser;
use serde::{Serialize, Deserialize};
use reqwest::header;
use crate::config::Config;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
struct GETAPIResponse {
    origin: String,
}


#[derive(Serialize, Deserialize, Debug)]
struct JSONResponse {
    json: HashMap<String, String>,
}

#[derive(Parser)]
#[command(author="Hemenguelbindi", version="0.0.1", about="Cli beholder from activiti in github", long_about = None)]
pub struct Cli {
    /// check last activity user
    pub name : String,
}


impl Cli {
    pub async  fn progress(&self)-> Result<(), Box<dyn std::error::Error>>{
        let config = Config::init();
        print_ascii_owl();
        println!("{}", self.name);
        let url = format!("https://api.github.com/users/{}/events", self.name);
        let mut headers = header::HeaderMap::new();
        headers.insert("Accept", "application/vnd.github+json".parse().unwrap());
        let token = format!("Bearer {}", config.token);
        headers.insert("Authorization", token.parse().unwrap());
        let client = reqwest::Client::new();
        let reqp200 = client.get(&url).headers(headers).send().await?.json::<GETAPIResponse>().await?;
        
        println!("{:#?}", reqp200);
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

