mod commands;
mod config;
use clap::Parser;
use commands::Cli;


#[tokio::main]
async fn main() {
    let args = Cli::parse();

    args.progress().await.unwrap();
}